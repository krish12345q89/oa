use heed::{ Database, Env, EnvOpenOptions };
use heed::types::SerdeBincode;
use crate::schema::application::Application;
use anyhow::{ Result, Context };
use std::path::Path;

#[derive(Clone)]
pub struct AppDatabase {
    env: Env,
    db: Database<SerdeBincode<String>, SerdeBincode<Application>>,
}

pub trait DBApplication: Send + Sync {
    fn save(&self, app: &Application) -> Result<()>;
    fn get(&self, id: &str) -> Result<Option<Application>>;
    fn delete(&self, id: &str) -> Result<()>;
    fn update(&self, app: &Application) -> Result<()>;
    fn list(&self) -> Result<Vec<Application>>;
}

impl AppDatabase {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        std::fs
            ::read_dir(&path)
            .is_err()
            .then(|| {
                tracing::warn!("Database directory does not exist, creating: {:?}", path.as_ref());
                std::fs::create_dir_all(&path).expect("Failed to create database directory");
            });

        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(1024 * 1024 * 1024) // 1GB
                .max_dbs(1)
                .open(path)?
        };

        let mut txn = env.write_txn()?;
        let db = env.create_database(&mut txn, Some("applications"))?;
        txn.commit()?;

        Ok(Self { env, db })
    }

    fn with_write_txn<F, T>(&self, f: F) -> Result<T> where F: FnOnce(&mut heed::RwTxn) -> Result<T> {
        let mut txn = self.env.write_txn()?;
        let result = f(&mut txn)?;
        txn.commit()?;
        Ok(result)
    }

    fn with_read_txn<F, T>(&self, f: F) -> Result<T> where F: FnOnce(&heed::RoTxn) -> Result<T> {
        let txn = self.env.read_txn()?;
        f(&txn)
    }
}

impl DBApplication for AppDatabase {
    fn save(&self, app: &Application) -> Result<()> {
        self.with_write_txn(|txn| {
            self.db.put(txn, &app.id, app).context("Failed to save application")
        })
    }
    fn update(&self, app: &Application) -> Result<()> {
        self.with_write_txn(|txn| {
            self.db.put(txn, &app.id, app).context("Failed to update application")
        })
    }
    fn get(&self, id: &str) -> Result<Option<Application>> {
        self.with_read_txn(|txn| {
            self.db.get(txn, &id.to_string()).context("Failed to get application")
        })
    }

    fn delete(&self, id: &str) -> Result<()> {
        self.with_write_txn(|txn| {
            // Convert &str to &String and map Result<bool, _> to Result<(), _>
            let id_string = id.to_string();
            self.db
                .delete(txn, &id_string)
                .context("Failed to delete application")
                .map(|_| ())
        })
    }

    fn list(&self) -> Result<Vec<Application>> {
        self.with_read_txn(|txn| {
            self.db
                .iter(txn)?
                .map(|res| {
                    let (_k, v) = res?;
                    Ok(v)
                })
                .collect::<heed::Result<Vec<_>>>()
                .context("Failed to list applications")
        })
    }
}
