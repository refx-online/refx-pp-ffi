use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_c() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            dll_name: "refxpp".to_string(),
            ..Config::default()
        },
        refxpp::my_inventory(),
    )
    .write_file("bindings/AkatsukiPPFFI.cs")?;

    Ok(())
}
