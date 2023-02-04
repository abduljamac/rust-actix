use actix_web::{post, web::Data, web::Json, HttpResponse};

use crate::model::task_model::Task;
use crate::repository::db::MongoRepo;

#[post("/task")]
pub async fn create_task(db: Data<MongoRepo>, new_task: Json<Task>) -> HttpResponse {
    let data = Task {
        id: new_task.id.clone(),
        task_type: new_task.task_type.clone(),
    };
    let task_details = db.create_task(data).await;
    match task_details {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
