
/* 
1. 创建待办
2. 更新待办
3. 删除待办
4. 获取待办列表
 */

use std::net::SocketAddr;
use axum::{
    routing::{
        get,
        put,
    },
    response::{
        Response,
        IntoResponse,
        Json,
    },
    Router, 
    extract::{State, Query, Path},
};
use serde::{Serialize,Deserialize};
use std::sync::{Arc,RwLock};
use std::collections::{HashMap};
// use uuid::Uuid;




#[tokio::main]
async fn main() {
    let db = Db::default();
    let app = Router::new().route("/", get(|| async {"hello world!"}))
        .route("/todos", get(todos_index).post(create_todos))
        .route("/todos/:id", put(update_todos).delete(delete_todos).get(get_todos))
        .with_state(db);

    let addr = SocketAddr::from(([127,0,0,1],3000));

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

}

#[derive(Clone,Serialize,Deserialize)]
struct Todos {
    id:u32,
    text:String,
    completed:bool,
}

#[derive(Default)]
struct TodosTable {
    index:u32,
    inner:HashMap<u32,Todos>
}
impl TodosTable {
    fn push(&mut self,todos:NewTodos)->Option<Todos>{
        self.index += 1;
        let new_todos = Todos{
            id: self.index,
            text:todos.text,
            completed:false
        };
        self.inner.insert(self.index,new_todos.clone());
        Some(new_todos)
    }

    fn get(&self,id:&u32)->Option<&Todos>{
        self.inner.get(&id)
    }

    fn delete(&mut self,id:&u32)->Option<Todos>{
        self.inner.remove(id)
    }

    fn update(&mut self,todos:Todos){
        self.inner.insert(todos.id, todos);
    }
}

type Db = Arc<RwLock<TodosTable>>;

#[derive(Serialize,Deserialize,Debug,Clone)]
struct NewTodos {
    text:String,
}

async fn create_todos(State(db):State<Db>,Json(payload): Json<NewTodos>) ->impl IntoResponse{
    let new_todos = db.write().unwrap().push(payload).unwrap();
    Json(new_todos)
}

#[derive(Serialize,Deserialize,Clone)]
struct UpdateTodos {
    text:String,
    completed:bool
}

async fn update_todos(Path(id):Path<u32>,State(db):State<Db>,Json(payload):Json<UpdateTodos>) ->impl IntoResponse{
    db.write().unwrap().update(Todos{
        id,
        text:payload.text,
        completed:payload.completed,
    });
    "update success"
}

async fn todos_index(State(db):State<Db>) ->impl IntoResponse {
    let db = db.write().unwrap();
    let resp = db.inner.values().cloned().collect::<Vec<Todos>>();
    Json(resp)
}

async fn delete_todos(Path(id):Path<u32>,State(db):State<Db>) ->Response {
    match db.write().unwrap().delete(&id) {
        Some(todos)=>{
            Json(todos).into_response()
        },
        None=>{
            "NOT FOUND".into_response()
        }
    }
}

async fn get_todos(Path(id):Path<u32>,State(db):State<Db>)->Response{
    match db.read().unwrap().get(&id) {
        Some(todos)=>{
            Json(todos).into_response()
        },
        None=>{
            "NOT FOUND".into_response()
        }
    } 
}


#[cfg(test)]
mod test{
    use std::collections::HashMap;
    
    #[test]
    fn test_hash_map(){
        let mut hash_map = HashMap::new();
        hash_map.insert(1, "Ok");
        assert_eq!(hash_map.iter().count(),1);
        let val = hash_map.get(&1).cloned();
        assert_eq!(val,Some("Ok"));
        assert_eq!(hash_map.iter().count(),1);

    }
}