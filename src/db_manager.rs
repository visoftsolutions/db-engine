use std::collections::HashSet;

use crate::db_class::DbClass;


pub struct DbManager{
    classes: HashSet<DbClass>
}

impl DbManager {
    pub fn new() -> Self{
        DbManager { classes: HashSet::new()}
    }

    pub fn add_class(&mut self, class: DbClass) {
        self.classes.insert(class);
    }
}