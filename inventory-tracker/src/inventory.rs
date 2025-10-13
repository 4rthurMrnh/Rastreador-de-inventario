use crate::models::{ErroEstoque, Produto};
use crate::database;
use std::fmt::{self, Display};

pub struct InventoryService {
    db: Database,
}

impl InventoryService {
    pub fn new(db: Database) -> Self {
        InventoryService { db }
    }
}


pub fn buscar_produto(id: i32) -> Result<Produto, String> {
    let produto_result = database::db_buscar_produto_por_id(id);
    map.err(|e| format!("Falha de conexão com o DB: {}", e))?;

    let produto = match produto_result {
        Some(p) => p,
        None => return Err(format!("Produto ID {} não encontrado ou não existe.", id)),
    };
    Ok(produto)
}


pub fn adicionar_estoque(produto: &mut Produto, quantidade: i32) -> Result<(), ErroEstoque> {
    if quantidade <= 0 {
        return Err(ErroEstoque::ValorInvalido);
    }
    produto.estoque += quantidade;
    Ok(())
}

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

pub fn realizar_venda(produto_id: i32, quantidade: i32) -> Result<Produto, String> {
    let pool = self.db.get_pool();
    let mut produto = self.db.db_buscar_produtos(pool, id).await
        .map_err(|e| format!("Falha no DB ao buscar: {}", e))?
        .ok_or_else(|| format!("Produto ID {} não encontrado.", id))?;

    Self::remover_estoque(&mut produto, quantidade)
        .map_err(|e| format!("Erro de venda: {}", e))?;

    self.db.db_salvar_produto(pool, &produto).await
        .map_err(|e| format!("Falha ao salvar no DB: {}", e))?;

    Ok(produto)
}
