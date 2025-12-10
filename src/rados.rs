use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    sync::Mutex,
};

use crate::{
    ByteCount, IoCtx, RadosError, Result,
    error::{maybe_err, maybe_err_or_val},
    librados::{
        LIBRADOS_VER_EXTRA, LIBRADOS_VER_MAJOR, LIBRADOS_VER_MINOR, rados_cluster_stat,
        rados_cluster_stat_t, rados_conf_parse_argv, rados_conf_parse_env, rados_conf_read_file,
        rados_conf_set, rados_connect, rados_create, rados_pool_list, rados_shutdown, rados_t,
        rados_version,
    },
};

/// File configurations that can be applied when connecting to a Rados cluster.
#[derive(Debug, Clone)]
pub enum FileConfig {
    /// Search the default file paths.
    ///
    /// From the librados documentation:
    ///
    /// * `$CEPH_CONF` (environment variable)
    /// * `/etc/ceph/ceph.conf`
    /// * `~/.ceph/config`
    /// * `ceph.conf` (in the current working directory)
    /// ```
    Default,
    /// Apply configuration from the file at the provided
    /// path.
    Path(String),
}

/// Configuration to be used when connecting to a Rados cluster.
///
/// The `Default` implementation returns [`RadosConfig::new(Some(FileConfig::Default)`](RadosConfig::new).
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
    /// Create a new [`RadosConfig`] using empty values
    /// for all fields, and set the file configuration to `files`.
    ///
    /// Additionally, it sets `rados_quiet` to `true`.
    pub fn new(file: Option<FileConfig>) -> Self {
        Self {
            id: None,
            files: file.into_iter().collect(),
            argv: Vec::new(),
            env: Vec::new(),
            rados_quiet: true,
        }
    }

    /// Set the ID to be used when connecting.
    ///
    /// # Panics
    /// This function panics if `id` contains an internal `0` byte.
    pub fn set_id<'a>(&'a mut self, id: Option<&str>) -> &'a mut Self {
        self.id = id.map(|v| CString::new(v).expect("User contained NUL byte"));
        self
    }

    /// Set the file configurations to search.
    pub fn set_files(&mut self, files: Vec<FileConfig>) -> &mut Self {
        self.files = files;
        self
    }

    /// Set the arguments to pass when connecting.
    ///
    /// For more info, see [`rados_conf_parse_argv`](https://docs.ceph.com/en/latest/rados/api/librados/#c.rados_conf_parse_argv).
    pub fn set_argv(&mut self, argv: Vec<String>) -> &mut Self {
        self.argv = argv;
        self
    }

    /// Set the environment variables to search.
    ///
    /// For more info, see [`rados_conf_parse_env`](https://docs.ceph.com/en/latest/rados/api/librados/#c.rados_conf_parse_env)
    pub fn set_env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = env;
        self
    }

    /// Configure librados to be quiet.
    ///
    /// If this is set to `false`, log messages from librados are
    /// printed to stdout.
    pub fn set_rados_quiet(&mut self, quiet: bool) -> &mut Self {
        self.rados_quiet = quiet;
        self
    }
}

/// Cluster statistics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClusterStats {
    #[doc = "total device size"]
    /// The total count of bytes of device bytes usable.
    pub size: ByteCount,
    /// The amount of bytes used in the cluster.
    pub used: ByteCount,
    /// The amount of bytes available.
    pub available: ByteCount,
    /// The number of objects.
    pub num_objects: u64,
}

impl ClusterStats {
    fn from(value: rados_cluster_stat_t) -> Self {
        Self {
            size: ByteCount::from_kb(value.kb),
            used: ByteCount::from_kb(value.kb_used),
            available: ByteCount::from_kb(value.kb_avail),
            num_objects: value.num_objects,
        }
    }
}

/// A connection to a rados cluster.
///
/// This struct maintains a connection, but most actual
/// object operations are performed using an [`IoCtx`]
/// (see [`Rados::create_ioctx`]).
#[derive(Debug)]
pub struct Rados(rados_t);

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
    /// Connect to the rados cluster using the provided configuration.
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

    /// Attempt to create a new [`IoCtx`] for rados pool `pool` in this cluster.
    pub fn create_ioctx(&mut self, pool: &str) -> Result<IoCtx<'_>> {
        IoCtx::new(self, pool)
    }

    /// Fetch statistics for the cluster.
    pub fn cluster_stats(&mut self) -> Result<ClusterStats> {
        let mut cluster_stats = MaybeUninit::uninit();

        maybe_err(unsafe { rados_cluster_stat(self.0, cluster_stats.as_mut_ptr()) })?;

        let stats = unsafe { cluster_stats.assume_init() };
        Ok(ClusterStats::from(stats))
    }

    /// List the pools available in the cluster.
    pub fn list_pools(&mut self) -> Result<Vec<String>> {
        let len =
            maybe_err_or_val(unsafe { rados_pool_list(self.inner(), std::ptr::null_mut(), 0) })?;

        let mut data = vec![0i8; len as usize];

        maybe_err(unsafe { rados_pool_list(self.inner(), data.as_mut_ptr(), data.len()) })?;

        let mut pools = Vec::new();
        let mut current_value = String::new();

        for byte in data.into_iter() {
            if byte == 0 && current_value.len() == 0 {
                break;
            } else if byte == 0 {
                pools.push(std::mem::take(&mut current_value));
            } else {
                current_value.push(byte as u8 as char);
            }
        }

        Ok(pools)
    }

    pub(crate) fn inner(&self) -> rados_t {
        self.0
    }
}

impl Drop for Rados {
    fn drop(&mut self) {
        unsafe { rados_shutdown(self.0) }
    }
}
