use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::transactions)]
pub struct Transaction {
    #[serde(default)]
    pub transactionId: String,
    pub accountNumber: u64,
    pub accountName: String,
    pub accountCategory: String,
    pub accountProvider: String,
    pub bankCode: String,
    pub password: String,
    pub tranAmount: i32,
    pub tranCategory: String,
    pub channel: String,
    pub currency: String,
    pub paymentDate: Option<chrono::NaiveDateTime>,
    pub tranSignature: String,
    pub narration: String,
}
