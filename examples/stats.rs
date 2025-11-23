use librados::{IoCtx, Rados, RadosConfig};

fn main() {
    let config = RadosConfig::default();
    let mut rados = Rados::connect(&config).unwrap();

    let stats = rados.cluster_stats().unwrap();

    if stats.used + stats.available != stats.size {
        eprintln!("Used + avaialble not equal to size.");
    }

    println!("{:#?}", stats);

    if let Some(pool) = std::env::args().nth(1) {
        let mut ctx = IoCtx::new(&mut rados, &pool).unwrap();
        println!("{:#?}", ctx.pool_stats().unwrap());
    } else {
        println!("No pool argument provided, not looking up pool stats.")
    }
}
