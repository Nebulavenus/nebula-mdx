#[macro_use]
extern crate log;

pub use mdlx::MDLXModel;

pub mod chunks;
pub mod consts;
mod mdlx;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn read_write_mdx_file_api() {
        init();

        let raw_data = fs::read("testfiles/druidcat.mdx").unwrap();
        let model = MDLXModel::read_mdx_file(raw_data.clone()).unwrap();
        dbg!(&raw_data.len());

        let bytes = MDLXModel::write_mdx_file(model).unwrap();
        dbg!(&bytes.len());
        fs::write("testfiles/resave.mdx", bytes).unwrap();
    }
}
