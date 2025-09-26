pub struct Produto {
    pub id: i32,
    pub nome: String,
    pub preco: f64,
}

pub struct VendaHistoricos {
    pub id: i32,
    pub produto_id: i32,
    pub data_venda: String,
    pub quantidade: i32,
}