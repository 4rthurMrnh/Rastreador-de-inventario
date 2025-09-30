use crate::models::{ErroEstoque, Produto};
use crate::database;

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
    let mut produto = database::buscar_produto_por_id(produto_id)
        .map_err(|e| format!("Falha na busca do produto: {:?}", e))?;

    remover_estoque(&mut produto, quantidade)
        .map_err(|e| format!("Erro ao realizar a venda: {:?}", e))?;

    database::salvar_produto(&produto)
        .map_err(|e| format!("Falha ao salvar no DB: {}", e))?;

    Ok(produto)
}