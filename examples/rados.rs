use std::{
    fs::File,
    io::{StdoutLock, Write},
    path::PathBuf,
};

use clap::{Parser, ValueEnum};
use librados::{Cursor, IoCtx, Rados, RadosConfig};

#[derive(Parser)]
enum Cli {
    Ls {
        /// The pool to operate on
        #[arg(short, long, env = "RADOS_POOL")]
        pool: String,

        #[arg(long, default_value = "object-iterator")]
        mode: LsMode,
    },
    Get(Get),
    #[command(name = "lspools")]
    LsPools,
    Df,
}

#[derive(Parser)]
pub struct Get {
    /// The pool to operate on
    #[arg(short, long, env = "RADOS_POOL")]
    pub pool: String,
    /// The object whose contents to get.
    object: String,
    /// The path to write the data to (defaults to stdout if not specified).
    /// Specifying `-` also writes to stdout.
    out_path: Option<PathBuf>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum LsMode {
    ObjectIterator,
    Cursor,
    CursorParallel,
}

impl Default for LsMode {
    fn default() -> Self {
        Self::ObjectIterator
    }
}

fn main() {
    let cli = Cli::parse();

    let config = RadosConfig::default();
    let mut rados = Rados::connect(&config).unwrap();

    match cli {
        Cli::Ls { mode, pool } => {
            let ioctx = IoCtx::new(&mut rados, &pool).unwrap();
            ls(&ioctx, mode)
        }
        Cli::Get(opt) => {
            let ioctx = IoCtx::new(&mut rados, &opt.pool).unwrap();
            get(&ioctx, opt)
        }
        Cli::LsPools => {
            let pools = rados.list_pools().unwrap();

            for pool in pools {
                println!("{pool}");
            }
        }
        Cli::Df => df(&mut rados),
    }
}

fn df(rados: &mut Rados) {
    let pools = rados.list_pools().unwrap();

    for pool in pools {
        let mut ioctx = IoCtx::new(rados, &pool).unwrap();
        let pool_stat = ioctx.pool_stats().unwrap();

        println!(
            "{pool} {} used, {} rd",
            pool_stat.user_bytes, pool_stat.object_bytes_read
        );
    }
}

enum FileOrStdout<'a> {
    File(File),
    StdOut(StdoutLock<'a>),
}

impl FileOrStdout<'_> {
    fn write_all(&mut self, data: &[u8]) -> std::io::Result<()> {
        match self {
            FileOrStdout::File(file) => file.write_all(data),
            FileOrStdout::StdOut(stdout_lock) => stdout_lock.write_all(data),
        }
    }
}

fn get(ioctx: &IoCtx<'_>, get: Get) {
    let stat = ioctx.stat_blocking(&get.object).unwrap();

    let mut out = if let Some(output) = get.out_path
        && output.as_os_str() != "-"
    {
        let file = File::create(output).unwrap();
        FileOrStdout::File(file)
    } else {
        FileOrStdout::StdOut(std::io::stdout().lock())
    };

    let mut left = stat.size.into_bytes();
    let mut offset = 0;

    while left > 0 {
        let to_read = (left.min(i32::MAX as u64)) as i32;
        let data = ioctx.read_blocking(&get.object, to_read, offset).unwrap();
        out.write_all(&data).unwrap();

        if data.len() == 0 {
            break;
        }

        left -= data.len() as u64;
        offset += data.len();
    }
}

fn ls(ioctx: &IoCtx<'_>, mode: LsMode) {
    match mode {
        LsMode::ObjectIterator => {
            for object in ioctx.objects().unwrap().map(|v| v.unwrap()) {
                println!("{}", object.oid());
            }
        }
        LsMode::Cursor => {
            let cursor = ioctx.object_cursor();
            print_cursor(cursor);
        }
        LsMode::CursorParallel => {
            let cursor = ioctx.object_cursor();

            std::thread::scope(|s| {
                cursor
                    .split(std::thread::available_parallelism().unwrap().get())
                    .for_each(|c| {
                        s.spawn(move || {
                            print_cursor(c);
                        });
                    });
            })
        }
    }
}

fn print_cursor(mut cursor: Cursor<'_, '_>) {
    loop {
        let values = cursor.read(1024).unwrap();

        if values.len() == 0 {
            break;
        }

        for value in values {
            println!("{}", value.oid());
        }
    }
}
