use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Thing,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub last_update: chrono::NaiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoFromUser {
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
}

impl TodoFromUser {
    pub async fn build(&self) -> Todo {
        Todo {
            id: Thing {
                tb: "todo".to_string(),
                id: Id::from(self.title.clone()),
            },
            title: self.title.clone(),
            content: self.content.clone(),
            completed: self.completed.unwrap_or(false),
            last_update: chrono::Local::now().naive_local(),
        }
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TodoCreate {
//     pub title: String,
//     pub content: String,
//     pub completed: bool,
//     pub created_at: chrono::NaiveDateTime,
//     pub updated_at: chrono::NaiveDateTime,
// }
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TodoUpdate {
//     pub title: String,
//     pub content: String,
//     pub completed: bool,
//     pub updated_at: chrono::NaiveDateTime,
// }
#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
