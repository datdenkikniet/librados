use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    sync::Mutex,
};

use crate::{
    ByteCount, RadosError, Result,
    error::maybe_err,
    librados::{
        LIBRADOS_VER_EXTRA, LIBRADOS_VER_MAJOR, LIBRADOS_VER_MINOR, rados_cluster_stat,
        rados_cluster_stat_t, rados_conf_parse_argv, rados_conf_parse_env, rados_conf_read_file,
        rados_conf_set, rados_connect, rados_create, rados_shutdown, rados_t, rados_version,
    },
};

#[derive(Debug, Clone)]
pub enum FileConfig {
    Default,
    Path(String),
}

#[derive(Debug, Clone)]
pub struct RadosConfig {
    id: Option<CString>,
    files: Vec<FileConfig>,
    argv: Vec<String>,
    env: Vec<String>,
    rados_quiet: bool,
}

impl Default for RadosConfig {
    fn default() -> Self {
        Self::new(Some(FileConfig::Default))
    }
}

impl RadosConfig {
    pub fn new(file: Option<FileConfig>) -> Self {
        Self {
            id: None,
            files: file.into_iter().collect(),
            argv: Vec::new(),
            env: Vec::new(),
            rados_quiet: true,
        }
    }

    pub fn set_id<'a>(&'a mut self, id: Option<&str>) -> &'a mut Self {
        self.id = id.map(|v| CString::new(v).expect("User contained NUL byte"));
        self
    }

    pub fn set_files(&mut self, files: Vec<FileConfig>) -> &mut Self {
        self.files = files;
        self
    }

    pub fn set_argv(&mut self, argv: Vec<String>) -> &mut Self {
        self.argv = argv;
        self
    }

    pub fn set_env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = env;
        self
    }

    pub fn set_rados_quiet(&mut self, quiet: bool) -> &mut Self {
        self.rados_quiet = quiet;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClusterStats {
    #[doc = "total device size"]
    pub size: ByteCount,
    #[doc = "total used"]
    pub used: ByteCount,
    #[doc = "total available/free"]
    pub available: ByteCount,
    #[doc = "number of objects"]
    pub num_objects: u64,
}

impl From<rados_cluster_stat_t> for ClusterStats {
    fn from(value: rados_cluster_stat_t) -> Self {
        Self {
            size: ByteCount::from_kb(value.kb),
            used: ByteCount::from_kb(value.kb_used),
            available: ByteCount::from_kb(value.kb_avail),
            num_objects: value.num_objects,
        }
    }
}

#[derive(Debug)]
pub struct Rados(pub(crate) rados_t);

unsafe impl Send for Rados {}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectError {
    Create(RadosError),
    ReadDefaultFileConfig(RadosError),
    ReadFileConfig(RadosError, String),
    ParseEnv(RadosError, String),
    ParseArgv(RadosError),
    Connect(RadosError),
}

impl Rados {
    pub fn connect(config: &RadosConfig) -> std::result::Result<Self, ConnectError> {
        let (mut maj, mut min, mut ext) = (0i32, 0i32, 0i32);

        unsafe { rados_version(&mut maj, &mut min, &mut ext) };

        if maj != LIBRADOS_VER_MAJOR as i32 {
            let env = std::env::var("LIBRADOS_SKIP_VERSION_CHECK").ok();
            let skip = match env.as_ref().map(|v| v.as_str()) {
                Some("yes") | Some("true") | Some("skip") => true,
                _ => false,
            };

            if !skip {
                panic!(
                    "The version of `librados` on this system ({maj}.{min}.{ext}) is incompatible with the version that the `librados` crate was built for ({LIBRADOS_VER_MAJOR}.{LIBRADOS_VER_MINOR}.{LIBRADOS_VER_EXTRA})."
                );
            }
        }

        let mut rados: rados_t = std::ptr::null_mut();
        let id = config
            .id
            .as_ref()
            .map(|v| v.as_ptr())
            .unwrap_or(std::ptr::null());

        maybe_err(unsafe { rados_create(&mut rados, id) }).map_err(ConnectError::Create)?;

        for file in config.files.iter() {
            match file {
                FileConfig::Default => {
                    let path = std::ptr::null();
                    maybe_err(unsafe { rados_conf_read_file(rados, path) })
                        .map_err(ConnectError::ReadDefaultFileConfig)?;
                }
                FileConfig::Path(in_path) => {
                    let path = CString::new(in_path.as_bytes()).unwrap();
                    maybe_err(unsafe { rados_conf_read_file(rados, path.as_ptr()) })
                        .map_err(|e| ConnectError::ReadFileConfig(e, in_path.into()))?;
                }
            }
        }

        for env in config.env.iter() {
            // A lock guarding usage of `rados_conf_parse_env`, as this
            // function is not thread-safe according to the librados
            // documentation.
            static PARSE_ENV_LOCK: Mutex<()> = Mutex::new(());

            let var = CString::new(env.as_bytes()).unwrap();
            let guard = PARSE_ENV_LOCK.lock().unwrap();
            maybe_err(unsafe { rados_conf_parse_env(rados, var.as_ptr()) })
                .map_err(|e| ConnectError::ParseEnv(e, env.to_string()))?;
            drop(guard);
        }

        if !config.argv.is_empty() {
            let argv: Vec<_> = config
                .argv
                .iter()
                .map(|v| CString::new(v.as_bytes()).unwrap())
                .collect();

            let mut argv_ptr: Vec<_> = argv.iter().map(|v| v.as_ptr()).collect();

            maybe_err(unsafe {
                rados_conf_parse_argv(rados, argv.len() as _, argv_ptr.as_mut_ptr())
            })
            .map_err(ConnectError::ParseArgv)?;

            drop(argv);
        }

        if config.rados_quiet {
            const VAL_FALSE: &'static CStr = c"false";
            const LOG_TO_STDERR: &'static CStr = c"log_to_stderr";
            const ERR_TO_STDERR: &'static CStr = c"err_to_stderr";

            let disable_log =
                unsafe { rados_conf_set(rados, LOG_TO_STDERR.as_ptr(), VAL_FALSE.as_ptr()) };
            assert!(
                disable_log == 0,
                "Failed to disable log_to_stderr: {disable_log}"
            );

            let disable_err =
                unsafe { rados_conf_set(rados, ERR_TO_STDERR.as_ptr(), VAL_FALSE.as_ptr()) };
            assert!(
                disable_err == 0,
                "Failed to disable err_to_stderr: {disable_err}"
            );
        }

        maybe_err(unsafe { rados_connect(rados) }).map_err(ConnectError::Connect)?;

        Ok(Self(rados))
    }

    pub fn cluster_stats(&mut self) -> Result<ClusterStats> {
        let mut cluster_stats = MaybeUninit::uninit();

        maybe_err(unsafe { rados_cluster_stat(self.0, cluster_stats.as_mut_ptr()) })?;

        let stats = unsafe { cluster_stats.assume_init() };
        Ok(stats.into())
    }
}

impl Drop for Rados {
    fn drop(&mut self) {
        unsafe { rados_shutdown(self.0) }
    }
}
