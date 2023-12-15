use std::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::transaction::Transaction;
use crate::repository::schema::transactions::dsl::*;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {

    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn make_payment(&self, transaction: Transaction) -> Result<Transaction, Error> {
        let transaction = Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..transaction
        };
        diesel::insert_into(transactions)
            .values(&transaction)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new transaction");
        Ok(transaction)
    }

    // pub fn get_todo_by_id(&self, todo_id: &str) -> Option<Todo> {
    //     let todo = todos
    //         .find(todo_id)
    //         .get_result::<Todo>(&mut self.pool.get().unwrap())
    //         .expect("Error loading todo by id");
    //     Some(todo)
    // }

}
