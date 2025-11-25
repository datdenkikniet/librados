use librados::{IoCtx, Rados, RadosConfig};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();

    let pool = args.nth(1).expect("pool as 1st argument");
    let object = args.next().expect("Object as 2nd argument");

    let config = RadosConfig::default();
    let mut rados = Rados::connect(&config).unwrap();
    let ctx = IoCtx::new(&mut rados, &pool).unwrap();

    println!("Getting xattr iterator");
    let mut attrs = ctx.get_xattrs(&object).await.unwrap();

    // `ExtendedAttributes` implements `Iterator`, but also supports
    // borrowing iteration through [`ExtendedAttributes::try_next`].
    while let Ok(Some((key, value))) = attrs.try_next() {
        let key = key.to_string_lossy();
        let single = ctx.get_xattr(&object, &key, value.len()).await.unwrap();
        assert_eq!(value, single);
        println!(
            "Found extended attribute `{key}` containing {} bytes",
            value.len()
        );
    }

    for (k, v) in ctx.get_omap_vals(&object).unwrap() {
        let key = std::str::from_utf8(&k).unwrap();

        println!("Found omap `{key}` of {} bytes.", v.len());
    }
}
