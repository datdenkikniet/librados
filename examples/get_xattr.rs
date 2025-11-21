use librsados::{IoCtx, Rados, RadosConfig};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();

    let pool = args.nth(1).expect("pool as 1st argument");
    let object = args.next().expect("Object as 2nd argument");
    let xattr = args.next().expect("xattr name as 3rd argument");

    let config = RadosConfig::default();
    let rados = Rados::connect(&config).unwrap();
    let mut ctx = IoCtx::new(&rados, &pool).unwrap();

    println!("Getting xattr {xattr} on object {object}");

    let mut xattr_buf = [0u8; 128];
    let xattr_len = ctx
        .get_xattr(&object, &xattr, &mut xattr_buf)
        .await
        .unwrap();

    println!("Succes: {:02X?}!", &xattr_buf[..xattr_len]);
}
