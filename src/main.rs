use std::io;

mod models;
use models::produto::Produto;

mod db;
use db::csv_handler::criar_catalogo;

mod services;
use services::busca::{busca_por_categoria, busca_por_cod_barras, busca_por_nome, busca_por_preco};

fn ler_input(mensagem: &str) -> String {
    let mut entrada = String::new();

    println!("{}", mensagem);
    io::stdin().read_line(&mut entrada).expect("Falha ao ler linha");

    entrada.trim().to_string()
}

fn main() {

    let catalogo = criar_catalogo();

    loop {
        let tipo_pesquisa = ler_input("\nQual tipo de pesquisa gostaria de fazer? (use caixa baixa)\n(C)odigo de (B)arras || (N)ome || (C)ategoria || (P)reço || (S)air");

        match tipo_pesquisa.trim() {
            "cb" => busca_por_cod_barras_input(&catalogo),
            "n" => busca_por_nome_input(&catalogo),                            //Nome
            "c" => busca_por_categoria_input(&catalogo),                       //Categoria
            "p" => busca_por_preco_input(&catalogo),
            "s" => {
                    println!("Encerrando o programa...");
                    return;
            },
            _ =>{
                    println!("Por favor, escolha dentre as opções fornecidas.");
            }
        }
    }
}

//Busca por nome
fn busca_por_nome_input(catalogo: &Vec<Produto>){

    //Pede o termo de busca no console e realiza a conversão em String
    let termo = ler_input("insira o nome do produto");

    //Faz a verificação de compatibilidade com o catalogo fornecido
    let resultados = busca_por_nome(&catalogo, &termo);

    if resultados.is_empty() {
        println!("Não existem produtos com essa descrição... A escrita está correta?");
    } else {
        println!("{} produto(s) encontrados para termo '{}':", resultados.len(), termo);
        for p in resultados {
            println!("{} - R$ {} - Qnt. {}", p.nome, p.preco, p.quant);
        }
    }
}

//Busca por categoria
fn busca_por_categoria_input(catalogo: &Vec<Produto>){

    //Pede o termo de busca no console e realiza a conversão em String
    let categoria = ler_input("insira a categoria dos produtos");

    let resultados = busca_por_categoria(&catalogo, &categoria);

    //Faz a verificação de compatibilidade com o catalogo fornecido
    if resultados.is_empty() {

        println!("Não existem produtos com essa descrinção... A escrita está correta?");
        
    } else {
        println!("{} Produtos encontrados na categoria {}", resultados.iter().count(), categoria);
        for p in resultados{
            println!("{} - Categoria: {} - R$ {} - Qnt. {}", p.nome, p.categoria, p.preco, p.quant);
        }
    }

}

//Busca por preço
fn busca_por_preco_input(catalogo: &Vec<Produto>){

    //Define o tipo do filtro por preço
    let tipo_filtro = ler_input("Qual tipo de filtragem gostaria de fazer? \n 1 -> Apatir de X || 2 -> Menor que X");

    let maior_que = match tipo_filtro.parse::<i32>() {
        Ok(1) => true,
        Ok(2) => false,
        _ =>{
            println!("Por favor, escolha dentre as opções fornecidas.");
            return;
        }
    };

    //Pede o termo de busca no console e realiza a conversão em String
    let preco: f32 = ler_input("Digite o valor de referência:").trim().parse().expect("Por favor, insira um número válido");

    let resultados = busca_por_preco(&catalogo, maior_que, &preco);

    if resultados.is_empty() {

        println!("Não existem produtos com essa descrinção... A escrita está correta?");
        
    } else {
        println!("{} Produtos se aplicam ao filtro 'buscar itens de valor {} que {}'",
                    resultados.iter().count(),
                    if maior_que {"maior"} else {"menor"}, 
                    preco);

        for p in resultados{
            println!("{} - R$ {} - Qnt. {}", p.nome, p.preco, p.quant);
        }
    }
}

//Busca por código de barras
fn busca_por_cod_barras_input(catalogo: & Vec<Produto>){

    //Pede o termo de busca no console e realiza a conversão em String
    let cod_barr = ler_input("insira o código de barras do produto\nATENÇÃO: O número deve ser digitado completamente. A busca pode demorar um pouco...");

    //Faz a verificação de compatibilidade com o catalogo fornecido
    if let Some(p) = busca_por_cod_barras(&catalogo, &cod_barr) {
        println!("{} - {} - R$ {} - Qnt. {}", p.cod_barras, p.nome, p.preco, p.quant);
    } else {
        println!("Não existem produtos com essa descrinção... A escrita está correta?");
    }
}