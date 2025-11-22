use librsados::{IoCtx, Rados, RadosConfig};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();

    let pool = args.nth(1).expect("pool as 1st argument");
    let object = args.next().expect("Object as 2nd argument");
    let xattr = args.next().expect("xattr name as 3rd argument");

    let config = RadosConfig::default();
    let rados = Rados::connect(&config).unwrap();
    let mut ctx1 = IoCtx::new(&rados, &pool).unwrap();
    let mut ctx2 = IoCtx::new(&rados, &pool).unwrap();

    println!("Getting xattr {xattr} on object {object}");

    let mut xattr_buf = [0u8; 128];

    let (xattr_len, xattrs) = tokio::join!(
        ctx1.get_xattr(&object, &xattr, &mut xattr_buf),
        ctx2.get_xattrs(&object)
    );

    println!("Succes: {:02X?}!", &xattr_buf[..xattr_len.unwrap()]);

    for (name, value) in xattrs.unwrap() {
        println!("Name: {}, value: {:02X?}", name, value);
    }
}
