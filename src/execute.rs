use anyhow::Result;

pub trait Executable {
    fn execute(&self) -> Result<()>;
}
