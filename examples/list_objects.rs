use librados::{Rados, RadosConfig};

fn main() {
    let mut args = std::env::args();

    let pool = args.nth(1).expect("pool as 1st argument");

    let config = RadosConfig::default();
    let mut rados = Rados::connect(&config).unwrap();
    let ctx = rados.create_ioctx(&pool).unwrap();

    let cursor = ctx.object_cursor();
    let cursors = cursor.split(std::thread::available_parallelism().unwrap().get());

    eprintln!("Counting all objects in parallel");

    std::thread::scope(move |s| {
        let results: Vec<_> = cursors
            .into_iter()
            .enumerate()
            .map(move |(idx, mut cursor)| {
                s.spawn(move || {
                    let mut total = 0;

                    eprintln!("Starting cursor {idx}");

                    loop {
                        let cursored = cursor.read(1024).unwrap();
                        total += cursored.len();

                        for object in cursored.iter() {
                            println!("{}", object.oid());
                        }

                        if cursored.is_empty() {
                            break;
                        }
                    }

                    eprintln!("Cursor {idx}: {total}");

                    total
                })
            })
            .collect();

        let mut total = 0;
        for res in results {
            total += res.join().unwrap();
        }

        eprintln!("Total objects in pool {pool}: {total}");
    });
}
