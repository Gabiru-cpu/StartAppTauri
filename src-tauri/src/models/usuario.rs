// use serde::Deserialize;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct Usuario {
    //pub id: Option<i32>,
    id: i32,
    nome: String,
    email: String,
    senha: String,
}