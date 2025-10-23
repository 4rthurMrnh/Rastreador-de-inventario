use crate::models::{ErroEstoque, Produto};
use crate::database::{Database, inventory_db::DbError}; // Importar a struct Database
use crate::models::VendaHistorico;
use crate::prediction; // Importar o módulo de previsão

pub struct InventoryService {
    db: Database,
}

impl InventoryService {
    pub fn new(db: Database) -> Self {
        InventoryService { db }
    }

    // O código de previsão deve estar dentro de um método.
    // `historico_vendas` precisa ser passado como argumento.
    pub async fn prever_demanda(&self, historico_vendas: &[VendaHistorico]) -> Result<f64, String> {
        let demanda_prevista = prediction::regressao_linear(historico_vendas)
            .map_err(|e| format!("Erro no modelo de previsão: {:?}", e))?;
        Ok(demanda_prevista)
    }

    // Esta função deve ser um método assíncrono do serviço.
    pub async fn buscar_produto(&self, id: i32) -> Result<Produto, String> {
        let produto = self.db.db_buscar_produtos(id).await
            .map_err(|e| format!("Falha de conexão com o DB: {}", e))?
            .ok_or_else(|| format!("Produto ID {} não encontrado ou não existe.", id))?;

        Ok(produto)
    }

    // Novo método para buscar histórico de vendas
    pub async fn buscar_historico_vendas(&self, produto_id: i32) -> Result<Vec<VendaHistorico>, DbError> {
        // Esta função precisaria ser implementada em `database.rs`
        // Por enquanto, retornamos um vetor vazio.
        self.db.db_buscar_historico_vendas(produto_id).await
    }

    // Este método não precisa de `self`, então pode ser um método associado (static).
    pub fn adicionar_estoque(produto: &mut Produto, quantidade: i32) -> Result<(), ErroEstoque> {
        if quantidade <= 0 {
            return Err(ErroEstoque::ValorInvalido);
        }
        produto.estoque += quantidade;
        Ok(())
    }

    // Este também pode ser um método associado.
    pub fn remover_estoque(produto: &mut Produto, quantidade: i32) -> Result<(), ErroEstoque> {
        if quantidade <= 0 {
            return Err(ErroEstoque::ValorInvalido);
        }
        if produto.estoque < quantidade {
            return Err(ErroEstoque::EstoqueInsuficiente);
        }
        produto.estoque -= quantidade;
        Ok(())
    }

    // Esta função deve ser um método assíncrono do serviço.
    pub async fn realizar_venda(&self, produto_id: i32, quantidade: i32) -> Result<Produto, String> {
        let mut produto = self.buscar_produto(produto_id).await?;

        Self::remover_estoque(&mut produto, quantidade)
            .map_err(|e| format!("Erro de venda: {:?}", e))?;

        self.db.db_salvar_produto(&produto).await
            .map_err(|e| format!("Falha ao salvar no DB: {}", e))?;

        Ok(produto)
    }
}
