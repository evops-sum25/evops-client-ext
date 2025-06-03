fn main() {
    #[cfg(feature = "uniffi")]
    uniffi::uniffi_bindgen_main();

    #[cfg(not(feature = "uniffi"))]
    {
        panic!("The feature `uniffi` should be enabled!");
    }
}
