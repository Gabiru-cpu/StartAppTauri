

use actix_web::{web, HttpResponse, Responder};
use sqlx::Row;
use crate::db::connect;
use crate::models::teste_data::TesteData;

#[actix_web::get("/data")]
async fn select_data() -> impl Responder {
    let pool = connect().await.unwrap();
    let result = sqlx::query("SELECT * FROM teste")
        .fetch_all(&pool)
        .await
        .map(|rows| {
            let mut result = String::new();
            for row in rows {
                let id: i32 = row.try_get(0).unwrap_or_default();
                let nome: &str = row.try_get(1).unwrap_or_default();
                let idade: i32 = row.try_get(2).unwrap_or_default();
                result.push_str(&format!("id: {}, nome: {}, idade: {}\n", id, nome, idade));
            }
            result
        })
        .unwrap_or_else(|_| "Failed to query database".to_string());

    HttpResponse::Ok().body(result)
}

#[actix_web::post("/data")]
async fn create_data(data: web::Json<TesteData>) -> impl Responder {
    let pool = connect().await.unwrap();
    let result = sqlx::query("INSERT INTO teste (nome, idade) VALUES ($1, $2) RETURNING id")
        .bind(data.nome())
        .bind(data.idade())
        .fetch_one(&pool)
        .await
        .map(|row| {
            let id: i32 = row.get(0);
            format!("Row with id {} created", id)
        })
        .unwrap_or_else(|_| "Failed to create row".to_string());

    HttpResponse::Ok().body(result)
}

#[actix_web::get("/data/{id}")]
async fn read_data(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner(); // Extrai o valor do path
    let pool = connect().await.unwrap();
    let result = sqlx::query("SELECT * FROM teste WHERE id = $1")
        .bind(id)
        .fetch_all(&pool)
        .await
        .map(|rows| {
            let mut result = String::new();
            for row in rows {
                let id: i32 = row.try_get(0).unwrap_or_default();
                let nome: &str = row.try_get(1).unwrap_or_default();
                let idade: i32 = row.try_get(2).unwrap_or_default();
                result.push_str(&format!("id: {}, nome: {}, idade: {}\n", id, nome, idade));
            }
            result
        })
        .unwrap_or_else(|_| "Failed to query database".to_string());

    HttpResponse::Ok().body(result)
}

#[actix_web::put("/data/{id}")]
async fn update_data(path: web::Path<i32>, data: web::Json<TesteData>) -> impl Responder {
    let pool = connect().await.unwrap();
    let id = path.into_inner(); // Armazena o valor de path em uma variável
    let result = sqlx::query("UPDATE teste SET nome = $1, idade = $2 WHERE id = $3")
        .bind(data.nome())
        .bind(data.idade())
        .bind(id) // Usa a variável id no lugar de path
        .execute(&pool)
        .await
        .map(|_| format!("Row with id {} updated", id))
        .unwrap_or_else(|_| "Failed to update row".to_string());

    HttpResponse::Ok().body(result)
}

#[actix_web::delete("/data/{id}")]
async fn delete_data(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner(); // Extrai o valor do path
    let pool = connect().await.unwrap();
    let result = sqlx::query("DELETE FROM teste WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| format!("Row with id {} deleted", id))
        .unwrap_or_else(|_| "Failed to delete row".to_string());

    HttpResponse::Ok().body(result)
}

#[actix_web::post("/hello")]
async fn hello_post() -> impl Responder {
    HttpResponse::Ok().body("Hello World! POST")
}

#[actix_web::delete("/hello")]
async fn hello_delete() -> impl Responder {
    HttpResponse::Ok().body("Hello World! DELETE")
}

#[actix_web::put("/hello")]
async fn hello_put() -> impl Responder {
    HttpResponse::Ok().body("Hello World! PUT")
}

#[actix_web::get("/hello")]
async fn hello_get() -> impl Responder {
    HttpResponse::Ok().body("Hello World! GET")
}