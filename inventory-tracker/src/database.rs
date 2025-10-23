use crate::models::Produto;
use std::error::Error;
use sqlx::{Error as SqlxError, PgPool};
use std::env; // Import the `env` module

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Database { pool }
    }
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn db_buscar_produtos(&self, id: i32) -> Result<Option<Produto>, SqlxError> {
        let produto_resultado = sqlx::query_as!(
            Produto,
            "SELECT id, nome, estoque, preco FROM produtos WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await;
        produto_resultado
    }

    pub async fn db_salvar_produto(&self, produto: &Produto) -> Result<(), SqlxError> {
        let query = "
        UPDATE produtos
        SET estoque = $1,
            nome = $2,
            preco = $3
            WHERE id = $4
        ";

        let resultado = sqlx::query(query)
        .bind(produto.estoque)
        .bind(&produto.nome)
        .bind(produto.preco)
        .bind(produto.id)
        .execute(&self.pool)
        .await;

        resultado?; // Check for error
        Ok(()) // Return Ok on success
    }
}

pub async fn establish_connection() -> Result<PgPool, Box<dyn Error>> {
    // Lê a DATABASE_URL do ambiente.
    // A chamada `dotenvy::dotenv()` no main.rs garante que o .env foi carregado.
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "A variável de ambiente DATABASE_URL precisa ser definida no arquivo .env")?;
        
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}
