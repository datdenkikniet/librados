use std::{fs::File, io::Write, path::PathBuf};

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

        println!("{pool} {} used, {} rd", pool_stat.user_bytes, pool_stat.object_bytes_read);
    }
}

fn get(ioctx: &IoCtx<'_>, get: Get) {
    let stat = ioctx.stat_blocking(&get.object).unwrap();

    let data = ioctx
        .read_blocking(&get.object, stat.size.into_bytes() as _, 0)
        .unwrap();

    if let Some(output) = get.out_path
        && output.as_os_str() != "-"
    {
        let mut file = File::create(output).unwrap();
        file.write_all(&data).unwrap();
    } else {
        std::io::stdout().write_all(&data).unwrap();
    }
}

fn ls(ioctx: &IoCtx<'_>, mode: LsMode) {
    match mode {
        LsMode::ObjectIterator => {
            for object in ioctx.objects().unwrap() {
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
