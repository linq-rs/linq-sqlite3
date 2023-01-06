use linq_rs::anyhow;

/// SQL prepare string generator
pub trait CodeGen {
    fn gen_code(&self) -> anyhow::Result<String>;
}
