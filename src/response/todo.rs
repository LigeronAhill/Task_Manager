use serde::Serialize;

use crate::model::todo::Todo;

#[derive(Serialize, Debug)]
pub struct TodoData {
    pub todo: Todo,
}
#[derive(Serialize, Debug)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: TodoData,
}
#[derive(Serialize, Debug)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Todo>,
}
