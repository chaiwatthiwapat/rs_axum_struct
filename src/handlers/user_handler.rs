use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

pub async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: "Rapi".to_string(), age: 35 },
        User { id: 2, name: "Anis".to_string(), age: 30 },
        User { id: 3, name: "Neon".to_string(), age: 22 },
    ];
    Json(users)
}
