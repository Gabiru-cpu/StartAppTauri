use sqlx::{ PgPool, postgres::PgPoolOptions};  // unused: Pool,

pub(crate) async fn connect() -> Result<PgPool, sqlx::Error> {
    let database_url = "postgres://meuusuario:minhasenha@localhost:5433/meudb";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

//criar tabelas
pub(crate) async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_USUARIO_TABLE).execute(pool).await?;
    sqlx::query(CREATE_CLIENTE_TABLE).execute(pool).await?;
    Ok(())
}
const CREATE_USUARIO_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS usuario (
        id SERIAL PRIMARY KEY,
        nome VARCHAR NOT NULL,
        email VARCHAR NOT NULL,
        senha VARCHAR NOT NULL
    )
"#;

const CREATE_CLIENTE_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS cliente (
        id SERIAL PRIMARY KEY,
        nome VARCHAR NOT NULL,
        valor_total DECIMAL NOT NULL,
        owner INT NOT NULL REFERENCES usuario(id)
    )
"#;