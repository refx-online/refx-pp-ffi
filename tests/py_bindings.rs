use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_c() -> Result<(), Error> {
    use interoptopus_backend_cpython::{Config, Generator};

    Generator::new(Config::default(), refxpp::my_inventory())
        .write_file("bindings/akatsuki_pp_ffi.py")?;

    Ok(())
}
