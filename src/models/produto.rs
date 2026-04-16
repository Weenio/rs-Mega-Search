use serde::{Serialize, Deserialize};

//Estrutura básica de um produto
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Produto {
    pub cod_barras: String, //Código de barras
    pub nome: String,       //Nome do produto
    pub categoria: String,  //Categoria do produto
    pub desc: String,       //Descrição breve
    pub preco: f32,         //Preço
    pub quant: u32,         //Quantidade em estoque
}