#[cfg(test)]
mod tests {
    use std::fs;

    use mega_search::services::{busca::busca_por_nome, index::indexar_busca};
    use ::mega_search::{db::csv_handler::criar_catalogo, models::produto::Produto, services::busca::busca_por_cod_barras};

    #[test]
    fn test_busca_por_cod_barras_encontra_item() {
        let catalogo = criar_catalogo();
        let cod_barras = "789123456006";
        let resultado = busca_por_cod_barras(&catalogo, cod_barras);
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().nome, "Monitor LG UltraWide 29");
    }

    #[test]
    fn test_busca_por_cod_barras_item_inexistente() {
        let catalogo = criar_catalogo();
        let cod_barras = "000000000000";
        let resultado = busca_por_cod_barras(&catalogo, cod_barras);
        assert!(resultado.is_none());
    }

    #[test]
    fn test_excluir_item_remove_do_catalogo() {
        let mut catalogo = criar_catalogo();
        let cod_barras = "7891000000028"; // Feijão Carioca 1kg
        catalogo.retain(|prod| prod.cod_barras != cod_barras);
        assert!(busca_por_cod_barras(&catalogo, cod_barras).is_none());
    }

    #[test]
    fn test_adicionar_item_incrementa_catalogo() {
        let mut catalogo = criar_catalogo();
        let tamanho_inicial = catalogo.len();

        let novo_produto = Produto {
            cod_barras: "789999999999".to_string(),
            nome: "Produto Teste".to_string(),
            categoria: "Teste".to_string(),
            desc: "Produto de teste".to_string(),
            preco: 99.9,
            quant: 10,
        };

        catalogo.push(novo_produto.clone());
        assert_eq!(catalogo.len(), tamanho_inicial + 1);
        assert!(busca_por_cod_barras(&catalogo, "789999999999").is_some());
    }

    #[test]
    fn test_indexacao_funciona() {
        let catalogo = criar_catalogo();

        let resultados = busca_por_nome(&catalogo, "arroz");

        if !resultados.is_empty() {
        if let Err(e) = indexar_busca("arroz", &resultados) {
            eprintln!("Falha ao salvar índice:\n{}", e);
            }
        }

        //Verifica se o arquivo foi criado
        assert!(std::path::Path::new("indexes/arroz_index.csv").exists());

        fs::remove_file("indexes/arroz_index.csv").ok();
    }
}