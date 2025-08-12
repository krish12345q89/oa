use std::error::Error;

use crate::{ lmdb::utils::DB, schema::order::Order };

pub trait DBOrder {
    fn insert(&self, order: Order) -> Result<(), Box<dyn Error>>;
    fn get_single(&self, id: String) -> Result<Option<Order>, Box<dyn Error>>;
    fn get(&self) -> Result<Option<Vec<Order>>, Box<dyn Error>>;
    fn put(&self, order: Order) -> Result<(), Box<dyn Error>>;
    fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}

impl DBOrder for DB {
    fn insert(&self, order: Order) -> Result<(), Box<dyn Error>> {
        let mut txn = self.env.write_txn()?;
        self.order_db.put(&mut txn, &order.id, &order)?;
        txn.commit()?;
        Ok(())
    }
    fn get_single(&self, id: String) -> Result<Option<Order>, Box<dyn Error>> {
        let txn = self.env.read_txn()?;
        if let Some(order) = self.order_db.get(&txn, &id)? {
            Ok(Some(order))
        } else {
            Ok(None)
        }
    }

    fn get(&self) -> Result<Option<Vec<Order>>, Box<dyn Error>> {
        let txn = self.env.read_txn()?;
        let mut orders = Vec::new();
        for result in self.order_db.iter(&txn)? {
            let (_, order) = result?;
            orders.push(order);
        }
        if orders.is_empty() {
            Ok(None)
        } else {
            Ok(Some(orders.clone()))
        }
    }
    fn put(&self, order: Order) -> Result<(), Box<dyn Error>> {
        let mut txn = self.env.write_txn()?;
        self.order_db.put(&mut txn, &order.id, &order)?;
        txn.commit()?;
        Ok(())
    }
    fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
        let mut txn = self.env.write_txn()?;
        self.order_db.delete(&mut txn, &id)?;
        txn.commit()?;
        Ok(())
    }
}
