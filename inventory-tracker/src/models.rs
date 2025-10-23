pub struct Produto {
    pub id: i32,
    pub nome: String,
    pub preco: f64,
    pub estoque: i32,
}

pub struct VendaHistorico { // Renamed from VendaHistoricos to VendaHistorico (singular)
    pub id: i32,
    pub produto_id: i32,
    pub data_venda: String,
    pub quantidade: i32,
}

#[derive(Debug)]
pub enum ErroEstoque {
    ProdutoNaoEncontrado,
    QuantidadeInsuficiente,
    ValorInvalido,
    EstoqueInsuficiente,
}