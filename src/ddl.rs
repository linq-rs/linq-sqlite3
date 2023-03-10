use linq_rs::{ddl::DDL, driver::DDLSupport, *};
use rdbc::Preparable;

use crate::{CodeGen, Driver};

#[async_trait]
impl DDLSupport for Driver {
    async fn exec_ddl<'a>(&mut self, ddls: &[ddl::DDL<'a>]) -> anyhow::Result<()> {
        let mut sqls = vec![];

        for ddl in ddls {
            sqls.push(ddl.gen_code()?);
        }

        let gen_ddl_sql = sqls.join(";");

        log::trace!(target:"LINQ_SQLITE3 DDL","{}",gen_ddl_sql);

        self.database.prepare(gen_ddl_sql).await?;

        Ok(())
    }
}

impl<'a> CodeGen for DDL<'a> {
    fn gen_code(&self) -> anyhow::Result<String> {
        match self {
            DDL::Alter(alter) => {}
            DDL::Create(create) => {}
            DDL::Drop(drop) => {}
            DDL::Truncate(truncate) => {}
        }
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Driver;

    use linq_rs::{anyhow, ddl, driver::DDLSupport};

    #[async_std::test]
    async fn test_ddl() -> anyhow::Result<()> {
        let mut driver = Driver::new("file:memdb_stmt?mode=memory&cache=shared")?;

        let table_name = "User";

        let qirs = ddl! {
            CREATE TABLE User(
                id INT PRIMARY,
                name STRING,
                date DATETIME,
                CONSTRAINT name_index UNIQUE(name),
                CONSTRAINT date_index INDEX(date),
            );

            CREATE TABLE Card(
                id INT PRIMARY AUTOINC,
                user_id INT,
                date DATETIME,
                CONSTRAINT user_id_foreign_key FOREIGN KEY (user_id) REFERENCES User(id),
            );

            // alter column
            ALTER TABLE Card ALTER COLUMN user_id BIGINT;

            ALTER TABLE Card ADD COLUMN card_no INT;

            ALTER TABLE Card RENAME COLUMN card_no TO no;

            ALTER TABLE Card DROP COLUMN no;

            // alter constraint

            ALTER TABLE Card ADD CONSTRAINT date_index INDEX(date);

            ALTER TABLE Card ALTER CONSTRAINT date_index UNIQUE(date);

            ALTER TABLE Card RENAME CONSTRAINT date_index TO date_unique;

            ALTER TABLE Card DROP CONSTRAINT date_unique;

            DROP TABLE Card;

            TRUNCATE TABLE #table_name;
        };

        driver.exec_ddl(&qirs).await?;

        Ok(())
    }
}
