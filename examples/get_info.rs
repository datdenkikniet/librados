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

    let blocking_omap = ctx.get_omap_vals_blocking(&object).unwrap();
    let async_omap = ctx.get_omap_vals(&object).await.unwrap();

    for ((k1, v1), (k2, v2)) in blocking_omap.zip(async_omap) {
        let k1 = std::str::from_utf8(&k1).unwrap();
        let k2 = std::str::from_utf8(&k2).unwrap();

        // This works, because `OmapKeyValues` yields an ordered list.
        assert_eq!(k1, k2);
        assert_eq!(v1, v2);

        println!("Found omap `{k1}` of {} bytes.", v1.len());
    }
}
