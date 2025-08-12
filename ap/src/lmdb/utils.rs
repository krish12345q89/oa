use heed::types::SerdeBincode;

use crate::schema::{ application::Application, order::Order, user::User };
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DB {
    pub env: heed::Env,
    pub order_db: heed::Database<SerdeBincode<String>, SerdeBincode<Order>>,
    pub user_db: heed::Database<SerdeBincode<String>, SerdeBincode<User>>,
    pub application_db: heed::Database<SerdeBincode<String>, SerdeBincode<Application>>,
}

pub async fn init_db<P: AsRef<std::path::Path>>(path: P) -> Result<DB, anyhow::Error> {
    let env = unsafe {
        heed::EnvOpenOptions
            ::new()
            .map_size(1024 * 1024 * 1024) // 1GB
            .max_dbs(3)
            .open(path)?
    };
    let new_env = env.clone();
    let mut txn = new_env.write_txn()?;
    let order_db = env
        .create_database(&mut txn, Some("orders"))
        .expect("Failed to create orders database");
    let user_db = env
        .create_database(&mut txn, Some("users"))
        .expect("Failed to create users database");
    let application_db = env
        .create_database(&mut txn, Some("applications"))
        .expect("Failed to create applications database");
    txn.commit()?;
    
    Ok(DB {
        env,
        order_db,
        user_db,
        application_db,
    })
}

