use linq_rs::driver::{QueryIterator, SelectSupport};
use linq_rs::*;

use crate::Driver;

#[async_trait]
impl<'a> SelectSupport<'a> for Driver {
    type SelectResult = SelectResult;
    async fn select(&mut self, selecter: &dml::Selecter<'a>) -> anyhow::Result<Self::SelectResult> {
        unimplemented!()
    }
}

pub struct SelectResult {}

#[async_trait]
impl QueryIterator for SelectResult {
    async fn next(&mut self) -> anyhow::Result<bool> {
        unimplemented!()
    }

    /// Get column value by offset id
    async fn get(&mut self, offset: usize) -> anyhow::Result<Variant> {
        unimplemented!()
    }

    /// Get column value by column name
    async fn get_by_name(&mut self, name: &str) -> anyhow::Result<Variant> {
        unimplemented!()
    }
}
