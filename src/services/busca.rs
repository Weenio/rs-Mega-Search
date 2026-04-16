use crate::{models::produto::Produto, services::index::{carregar_index, indexar_busca}};

//Busca simples por nome
pub fn busca_por_nome<'a>(catalogo: &'a Vec<Produto>, termo: &str) -> Vec<Produto> {

    let nome_index = termo.to_lowercase();

    // tenta carregar índice
    if let Some(resultados) = carregar_index(&nome_index) {
        println!("Usando índice já existente para termo '{}'", termo);
        return resultados;
    }

    //Se não existe realiza o scan completo
    let mut resultados: Vec<Produto> = Vec::new();

    for produto in catalogo {
        if produto.nome.to_lowercase().contains(&termo.to_lowercase()) {
            resultados.push(produto.clone()); // clone para salvar
        }
    }

    //salva índice para futuras buscas
    if !resultados.is_empty() {
        if let Err(e) = indexar_busca(&nome_index, &resultados) {
            eprintln!("Falha ao salvar índice:\n{}", e);
        }
    }

    //Retorna os valores após tentar salvar o indice
    resultados
} 

//busca por categorias
pub fn busca_por_categoria<'a>(catalogo: &'a Vec<Produto>, categoria: &str) -> Vec<Produto> {
    let mut resultados: Vec<Produto> = Vec::new();
    let nome_index = categoria.to_lowercase();

    // tenta carregar índice
    if let Some(resultados) = carregar_index(&nome_index) {
        println!("Usando índice já existente para a categoria '{}'", categoria);
        return resultados;
    }

    //Se não existe realiza o scan completo
    for produto in catalogo{                            //roda todo o catálogo de produtos disponibilizado
        if produto.categoria.eq_ignore_ascii_case(categoria){     //SE o nome do produto tiver o termo ultilizado para busca...
            resultados.push(produto.clone());                     //... Retorna os produtos que se aplicam ...
        }
    }

    //salva índice para futuras buscas
    if !resultados.is_empty() {
        if let Err(e) = indexar_busca(&nome_index, &resultados) {
            eprintln!("Falha ao salvar índice:\n{}", e);
        }
    }

    //Retorna os valores após tentar salvar o indice
    resultados
}

//Busca por preço. Inclue >x e <x
pub fn busca_por_preco<'a>(catalogo: &'a Vec<Produto>, maior_que: bool, preco: &f32) -> Vec<Produto> {
    let mut resultados = Vec::new();
    let nome_index = format!("{}__{}",
                                        if maior_que {"maior"} else {"menor"},
                                        preco.to_string());

    // tenta carregar índice
    if let Some(resultados) = carregar_index(&nome_index) {
        println!("Usando índice já existente para a busca 'itens de valor {} que {}'",
                    if maior_que {"maior"} else {"menor"}, 
                    preco);
        return resultados;
    }

    //Se não existe realiza o scan completo
    for produto in catalogo{                            //roda todo o catálogo de produtos disponibilizado
        
        if maior_que{                                             //Se o critério for "maior que x", enpacota os resultados
            if produto.preco >= *preco{
                resultados.push(produto.clone());
            }
        } else {                                                  //Se o critério for "menor que x", enpacota os resultados
            if produto.preco <= *preco{
                resultados.push(produto.clone());
            }
        }
    }

    //salva índice para futuras buscas
    if !resultados.is_empty() {
        if let Err(e) = indexar_busca(&nome_index, &resultados) {
            eprintln!("Falha ao salvar índice:\n{}", e);
        }
    }

    return resultados;                                             //Envia os resultados
}

//Busca por cód de barras
pub fn busca_por_cod_barras<'a>(catalogo: &'a Vec<Produto>, cod_barr: &str) -> Option<&'a Produto> {
    //Desncessária a indexação de itens por código de barras, uma vez que essa é uma busca por chave

    for produto in catalogo{                      //roda todo o catálogo de produtos disponibilizado

        if produto.cod_barras == cod_barr{                  //SE o nome do produto tiver o termo ultilizado para busca...
            return Some(produto);                           //... Retorna os produtos que se aplicam ...
        }

    }

    None                                                    //... CASO a busca falhe em retornar valores, retorna um valor "None".
} 
