use std::path::Path;
use csv::ReaderBuilder;
use crate::models::produto::Produto;

//Gera o catalogo inicial com base em uma Base de dados disponível, caso disponível.
pub fn criar_catalogo() -> Vec<Produto> {
    let caminho = "DataBases/base1_db.csv";

    if !Path::new(&caminho).exists() {
        return Vec::new(); // índice não existe
    }

    let mut rdr = ReaderBuilder::new()
        .from_path(caminho)
        .expect("Erro ao abrir base de dados CSV");

    let mut resultados = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(produto) => resultados.push(produto),
            Err(e) => eprintln!("Erro ao ler linha: {}", e),
        }
    }

    resultados
}