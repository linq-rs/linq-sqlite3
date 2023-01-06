use linq_rs::anyhow;
use rdbc::{future::DataSource, Database};
use rdbc_sqlite3::Sqlite3Driver;

#[allow(unused)]
/// sqlite3 allinone linq_rs driver
pub struct Driver {
    pub(crate) datasource: DataSource,
    pub(crate) database: Database,
}

impl Driver {
    pub fn new(url: &str) -> anyhow::Result<Self> {
        let datasource: DataSource = Default::default();

        datasource.register("sqlite3", Sqlite3Driver {})?;

        let database = datasource.open("sqlite3", url)?;

        Ok(Self {
            datasource,
            database,
        })
    }
}

mod ddl;
#[allow(unused)]
mod dml;

pub use ddl::*;
pub use dml::*;

mod codegen;
pub use codegen::*;
