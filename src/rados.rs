use std::{
    ffi::{CStr, CString},
    sync::Mutex,
};

use crate::librados::{
    rados_conf_parse_argv, rados_conf_parse_env, rados_conf_read_file, rados_conf_set,
    rados_connect, rados_create, rados_shutdown, rados_t,
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

    pub fn with_id(&mut self, id: Option<&str>) -> &mut Self {
        self.id = id.map(|v| CString::new(v).expect("User contained NUL byte"));
        self
    }

    pub fn with_files(mut self, files: Vec<FileConfig>) -> Self {
        self.files = files;
        self
    }

    pub fn with_argv(mut self, argv: Vec<String>) -> Self {
        self.argv = argv;
        self
    }

    pub fn with_env(mut self, env: Vec<String>) -> Self {
        self.env = env;
        self
    }

    pub fn with_rados_quiet(mut self, quiet: bool) -> Self {
        self.rados_quiet = quiet;
        self
    }
}

#[derive(Debug)]
pub struct Rados(pub(crate) rados_t);

unsafe impl Send for Rados {}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectError {
    Create,
    ReadDefaultFileConfig,
    ReadFileConfig(String),
    ParseEnv(String),
    ParseArgv,
    Connect,
}

impl Rados {
    pub fn connect(config: &RadosConfig) -> Result<Self, ConnectError> {
        let mut rados: rados_t = std::ptr::null_mut();
        let id = config
            .id
            .as_ref()
            .map(|v| v.as_ptr())
            .unwrap_or(std::ptr::null());

        let create = unsafe { rados_create(&mut rados, id) };

        if create != 0 {
            return Err(ConnectError::Create);
        }

        for file in config.files.iter() {
            match file {
                FileConfig::Default => {
                    let path = std::ptr::null();
                    let read = unsafe { rados_conf_read_file(rados, path) };

                    if read != 0 {
                        return Err(ConnectError::ReadDefaultFileConfig);
                    }
                }
                FileConfig::Path(path) => {
                    let path = CString::new(path.as_bytes()).unwrap();
                    let read = unsafe { rados_conf_read_file(rados, path.as_ptr()) };

                    if read != 0 {
                        return Err(ConnectError::ReadFileConfig(path.to_string_lossy().into()));
                    }
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
            let read = unsafe { rados_conf_parse_env(rados, var.as_ptr()) };
            drop(guard);

            if read != 0 {
                return Err(ConnectError::ParseEnv(var.to_string_lossy().into()));
            }
        }

        if !config.argv.is_empty() {
            let argv: Vec<_> = config
                .argv
                .iter()
                .map(|v| CString::new(v.as_bytes()).unwrap())
                .collect();

            let mut argv_ptr: Vec<_> = argv.iter().map(|v| v.as_ptr()).collect();

            let read =
                unsafe { rados_conf_parse_argv(rados, argv.len() as _, argv_ptr.as_mut_ptr()) };
            drop(argv);

            if read != 0 {
                return Err(ConnectError::ParseArgv);
            }
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

        let connect = unsafe { rados_connect(rados) };

        if connect != 0 {
            return Err(ConnectError::Connect);
        }

        Ok(Self(rados))
    }
}

impl Drop for Rados {
    fn drop(&mut self) {
        unsafe { rados_shutdown(self.0) }
    }
}
