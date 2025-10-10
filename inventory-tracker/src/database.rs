use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub struct Database {
    pool : PgPool,
}

impl Database {
use crate::models::Produto;
use sqlx::{Error as SqlxError, PgPool};

    pub async fn db_buscar_produtos(&self, id: i32) -> Result<Option<Produto>, SqlxError> {
        let produto_resultado = sqlx::query_as!(
            Produto,
            "SELECT id, nome, estoque, preco FROM produtos WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
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
    }

    pub fn new(pool: PgPool) -> Self {
        Database { pool }
    }
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

pub async fn establish_connection(url_db: &str) -> Result<PgPool, SqlxError> {
    PgPool::connect(DATABASE_URL).await
}
