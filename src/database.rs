use crate::*;

use sled::Db;

pub(crate) struct Database {
    db: Db,
}

impl Database {
    pub(crate) fn open(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Database { db })
    }

    pub(crate) fn get_or_store_salt(&mut self, email: &str, message_id: &str) -> Result<String> {
        match self.db.get(email).unwrap() {
            Some(salt) => Ok(std::str::from_utf8(&salt)?.to_string()),
            None => {
                self.db.insert(email, message_id)?;
                Ok(message_id.to_string())
            }
        }
    }
}
