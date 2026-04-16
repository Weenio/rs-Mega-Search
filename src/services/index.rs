use std::{fs::create_dir_all, path::Path};

use crate::models::produto::Produto;

//Função para realizar a indexação da busca.
pub fn indexar_busca(chave_index: &str, resultados: &Vec<Produto>) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all("indexes")?;
    
        let caminho = format!("indexes/{}_index.csv", chave_index.replace(" ", "_"));
        let mut wtr = csv::Writer::from_path(caminho)?;
        for produto in resultados {
            wtr.serialize(produto)?;
        }
        wtr.flush()?;
        Ok(())
}

//Função para carregar itens indexados
pub fn carregar_index(chave_index: &str) -> Option<Vec<Produto>> {
    let caminho = format!("indexes/{}_index.csv", chave_index.replace(" ", "_"));

    if !Path::new(&caminho).exists() {
        return None; // índice não existe
    }

    let mut rdr = csv::Reader::from_path(caminho).ok()?;
    let mut resultados = Vec::new();

    for result in rdr.deserialize() {
        if let Ok(produto) = result {
            resultados.push(produto);
        }
    }

    Some(resultados)
}
