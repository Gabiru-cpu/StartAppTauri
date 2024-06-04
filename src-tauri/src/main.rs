// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod controllers;

use actix_web::{ App, HttpServer};
use std::thread;
use std::env;

use controllers::teste_data_controller::*;


// Função para iniciar o servidor Actix-web em uma nova thread
fn start_backend() {
    thread::spawn(|| {
        actix_web::rt::System::new().block_on(async {
            
            //criar tabela user e clientes
            let pool = db::connect().await.unwrap();
            db::create_tables(&pool).await.unwrap();

            HttpServer::new(|| {
                App::new()
                    .service(hello_post)
                    .service(hello_delete)
                    .service(hello_put)
                    .service(hello_get)
                    .service(select_data)
                    .service(create_data)
                    .service(read_data)
                    .service(update_data)
                    .service(delete_data)
            })
            .bind("127.0.0.1:1421")
            .unwrap_or_else(|_| panic!("Can not bind to port 8080"))
            .run()
            .await
            .unwrap_or_else(|_| panic!("Failed to run server"));
        });
    });
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // Iniciar o backend em uma nova thread
    start_backend();

    // Inicializar o aplicativo Tauri
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
