/// Runs code in a temp dir.
#[macro_export]
macro_rules! run_in_temp_dir {
    ($block:block) => {
        let tmp_dir = tempfile::TempDir::new().unwrap();
        std::env::set_current_dir(&tmp_dir).unwrap();

        $block;
    };
}

#[macro_export]
macro_rules! test_file {
    ($filename:expr) => {
        use std::path::PathBuf;
        let manifest_dir =
            &std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR Missing");
        let mut orig_name = PathBuf::from(manifest_dir);
        orig_name.push("test-files");
        orig_name.push($filename);
        std::fs::copy(orig_name, $filename).unwrap();
    };
}
