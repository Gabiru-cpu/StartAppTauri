// use serde::Deserialize;

#[derive(sqlx::FromRow)]
#[allow(dead_code)] // marcador para tirar warning de nao uso
struct Cliente {
    id: i32,
    nome: String,
    valor_total: f64,
    owner: i32, // ID do usuário que criou o cliente
}