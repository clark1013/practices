use crate::sql::schema::Column;

// create table t (id int not null primary key, b varchar(15) not null default "");
// insert into t (id, b) values (1, "a"), (2, "b");
// select * from t;

pub enum Statement {
    CreateTable {
        name: String,
        columns: Vec<Column>
    }
}