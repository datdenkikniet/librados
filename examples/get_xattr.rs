use librsados::{IoCtx, Rados, RadosConfig};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();

    let pool = args.nth(1).expect("pool as 1st argument");
    let object = args.next().expect("Object as 2nd argument");

    let config = RadosConfig::default();
    let rados = Rados::connect(&config).unwrap();
    let mut ctx = IoCtx::new(&rados, &pool).unwrap();

    println!("Getting xattr iterator");
    let attrs: Vec<_> = ctx.get_xattrs(&object).await.unwrap().collect();

    for (key, value) in attrs {
        let mut buffer = vec![0u8; value.len()];
        let single_attr_len = ctx.get_xattr(&object, &key, &mut buffer).await.unwrap();

        let single = &buffer[..single_attr_len];

        assert_eq!(&value, single);

        println!(
            "Found extended attribute `{key}` containing {} bytes",
            value.len()
        );
    }
}
