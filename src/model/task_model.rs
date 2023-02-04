use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: String,
    pub task_type: String,
}
