use crate::parser::Statement;

pub struct Binder {}

impl Binder {
    pub fn new() -> Self {
        Binder {}
    }

    pub fn bind(&self, stmt: &Statement) {
        match stmt {
            Statement::CreateTable { .. } => {
                self.bind_create_table(stmt);
            }
            _ => panic!("mismatched statement type"),
        }
    }
}

impl Binder {
    pub fn bind_create_table(&self, stmt: &Statement) {
        match stmt {
            Statement::CreateTable {
                name,
                columns,
                constraints,
                ..
            } => {
                println!("{:?}", name);
                println!("{:?}", columns);
                println!("{:?}", constraints);
            }
            _ => panic!("mismatched statement type"),
        }
    }
}
