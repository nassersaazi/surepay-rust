use actix_web::{web, get, post, delete, put, HttpResponse};
//use crate::{models::todo::Todo, repository::database::Database};
use crate::{models::transaction::Transaction, repository::database::Database};


// #[post("/todos")] pub async fn create_todo(db: web::Data<Database>, new_todo: web::Json<Todo>) -> HttpResponse {
//     let todo = db.create_todo(new_todo.into_inner());
//     match todo {
//         Ok(todo) => HttpResponse::Ok().json(todo),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

#[post("/makePayment")]
pub async fn make_payment(db: web::Data<Database>, new_transaction: web::Json<Transaction>) -> HttpResponse {
    let transaction = db.make_payment(new_payment.into_inner());
    match transaction {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(make_payment)
    );
}
