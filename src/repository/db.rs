use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::model::task_model::Task;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client, Collection};

pub struct MongoRepo {
    col: Collection<Task>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<Task> = db.collection("task");
        MongoRepo { col }
    }

    pub async fn create_task(&self, task: Task) -> Result<InsertOneResult, Error> {
        let new_doc = Task {
            id: task.id,
            task_type: task.task_type,
        };
        let task = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating task");
        Ok(task)
    }
}
