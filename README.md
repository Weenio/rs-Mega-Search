# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

O projeto "Mega Search" é uma aplicação de console desenvolvida em Rust que visa resolver uma problemática fictícia: a empresa "MegaStore" enfrentava o sério problema no qual seu sistema de busca atual era muito lento, o que levava muitos clientes a abandonarem as compras por causa da lentidão durante a interação com o sistema.

A solução para esse problema foi desenvolver um algoritmo de buscas mais rápido, que pudesse lidar com milhares de itens.

A versão disponibilizada nesse diretório não tem suporte para banco de dados externo, então toda a base de dados é alocada localmente, assim como os índices de pesquisa.


# Tecnologias utilizadas

Para o desenvolvimento do aplicativo, foram utilizadas as bibliotecas **csv 1.3** e **serde 1.0**, pois os dados são guardados e indexados internamente pelo programa em tabelas csv.

## Como realizar buscas

Ao realizar a compilação do projeto e rodar ele em um console, as primeiras opções que aparecerão serão essas:

```<Bash>

Qual tipo de pesquisa gostaria de fazer? (use caixa baixa)
(C)odigo de (B)arras || (N)ome || (C)ategoria || (P)reço || (S)air

```
O usuário pode então escolher o meio de pesquisa digitando as letras destacadas em caixa baixa como resposta no console. Cada opção realiza a pesquisa de uma maneira diferente, sendo elas:
-   **Código de Barras** serve para realizar a busca por um item específico. Essa busca é feita apenas quando o usuário que um item. Por exemplo, buscar por “7891000000011” retorna apenas o “Arroz Branco 5kg” correspondente, e não o que possui o código de barras “7892000000103”. Apesar de não utilizar indexação, a busca por código de barras retorna apenas um único item, o que reduz o volume de dados processados.
	   ```<Bash>
		7891000000011,Arroz Branco 5kg,Alimentos,Arroz tipo 1 pacote 5kg,24.9,50		//O item retornado será esse
		7892000000103,Arroz Branco 5kg,Alimentos,Arroz tipo 1 pacote 5kg,24.9,50
	```

-   **Nome** faz a busca por itens que tenham a string fornecida pelo usuário. Dessa forma, uma busca por “note” consegue sim fornecer o resultado “Notebook 8GB RAM 256GB SSD”, mesmo que o usuário não tenha escrito exatamente o nome do item, letra por letra. Realizar essa pesquisa gera um index com a string fornecida.
    

-   **Categoria** funciona de forma igual à Nome. A diferença é que, dessa vez, o a string deve ser escrita de forma correta e em sua totalidade. Se o usuário realizar a pesquisa por “ele” esperando que o sistema retorne tanto itens da categoria “Eletrodomésticos” quanto “Eletrônicos”, o sistema não vai atender o pedido. Realizar essa pesquisa gera um index para o tipo.
    

-   **Preço** permite ao usuário realizar a pesquisa por itens de valor maior ou menor que o desejado. Quando a pesquisa for para um item de valor menor que 500R$ por exemplo, as entradas no console ficam de forma semelhante a isso:
	```<Bash>
		Qual tipo de pesquisa gostaria de fazer? (use caixa baixa)
		(C)odigo de (B)arras || (N)ome || (C)ategoria || (P)reço || (S)air
		
		p
		Qual tipo de filtragem gostaria de fazer? 
		 1 -> Apatir de X || 2 -> Menor que X     
		 
		2
		Digite o valor de referência:
		500
		
		55 Produtos se aplicam ao filtro 'buscar itens de valor menor que 500'
		Arroz Branco 5kg - R$ 24.9 - Qnt. 50
		Feijão Carioca 1kg - R$ 8.5 - Qnt. 80
		Açúcar Refinado 1kg - R$ 4.2 - Qnt. 100
		Café Torrado 500g - R$ 12.9 - Qnt. 60
		Leite Integral 1L - R$ 5.3 - Qnt. 120
		Macarrão Espaguete 500g - R$ 3.8 - Qnt. 90
		Óleo de Soja 900ml - R$ 6.7 - Qnt. 70
		Sal Refinado 1kg - R$ 2.5 - Qnt. 110
		Manteiga 200g - R$ 9.9 - Qnt. 40
		//...
	```
É aconselhado que, caso haja mudanças na base de dados, os índices criados em versões anteriores sejam excluídos. Os índices que o programa cria são apenas atalhos para otimização das buscas, e não são atualizados dinamicamente.

## Como realizar os testes

Você pode rodar os teste unitários do sistema apenas digitando ```cargo test``` estando no diretório da aplicação. Os testes unitários estão armazenados na pasta ```tests```, dentro do arquivo ```integracao.rs```

Os testes cobrem:  
  
- Busca por código de barras (sucesso e falha)  
- Inserção de novos itens  
- Remoção de itens em memória  
- Persistência de índices em disco  
  
Os testes de integração simulam o comportamento real do sistema.

## Estratégia de otimização  
  
O sistema utiliza uma abordagem híbrida:  
  
- Primeira busca: varredura completa (full scan)  
- Buscas subsequentes: uso de índices persistidos em CSV  
  
Isso reduz drasticamente o tempo de resposta para consultas repetidas.

## Limitações  
  
- Os índices não são atualizados automaticamente após alterações na base de dados  
- O uso de CSV como armazenamento limita a escalabilidade em ambientes reais  
- O sistema não possui paralelismo nas buscas
- 
## Arquitetura do sistema

São 3 módulos: “db”, “models” e “services”. Todos estão disponíveis dentro da pasta "src"

### Em ```src/db```
Dentro do arquivo “csv_handler.rs”, encontramos o código fonte para o carregamento do catálogo (em csv) para dentro de uma variável que é exportada na função como resposta. Sempre que a função “criar_catalogo()” é chamada dentro do código, sua resposta é expressamente um vetor de Produto (um modelo que será explicado mais tarde). Essa função tem seu acesso disponível pelo módulo “db”.

```<Rust>
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
```

### Em ```src/models```

“produto.rs” tem o modelo padrão para a construção de um objeto produto dentro do programa. Um produto é composto, necessariamente, pelas seguintes características:

```<Rust>
	use serde::{Serialize, Deserialize};
	
	//Estrutura básica de um produto
	#[derive(Debug, Serialize, Deserialize, Clone)]
	pub  struct  Produto {
		pub  cod_barras:  String,			//Código de barras
		pub  nome:  String,					//Nome do produto
		pub  categoria:  String,			//Categoria do produto
		pub  desc:  String,					//Descrição breve
		pub  preco:  f32,					//Preço
		pub  quant:  u32,					//Quantidade em estoque
	}
```

Desse modo, os produtos são padronizados de acordo com essa estrutura. Caso dentro do csv haja um campo extra ou faltando, o produto deserializado referente a essa linha retornará um erro. Caso seja requisitado para o sistema gerar um produto que não atenda esses requisitos, também haverá um erro.

### Em ```src/services```

Em “services”, temos dois módulos muito importantes. “busca.rs” é responsável expressamente pelas buscas. Nele, pode-se encontrar a lógica por trás das buscas explicadas anteriormente (Por código de barras, Nome, Preço etc.). No “index.rs” a lógica de indexação das buscas.

## Algoritmos e estruturas de dados

### Código de Barras
Na pesquisa por **código de barras** o trecho do código que realiza a busca pelo item é o seguinte:

```<Rust>
pub fn busca_por_cod_barras<'a>(catalogo: &'a Vec<Produto>, cod_barr: &str) -> Option<&'a Produto> {
    //Desncessária a indexação de itens por código de barras, uma vez que essa é uma busca por chave

    for produto in catalogo{                      //roda todo o catálogo de produtos disponibilizado

        if produto.cod_barras == cod_barr{                  //SE o nome do produto tiver o termo ultilizado para busca...
            return Some(produto);                           //... Retorna os produtos que se aplicam ...
        }

    }

    None                                                    //... CASO a busca falhe em retornar valores, retorna um valor "None".
} 
```

A busca por itens pelos códigos de barras é o equivalente a uma busca por itens por chave primária. Nesse caso, a ```&str``` fornecida DEVE ser igual a que existe na base de dados. Como dito anteriormente, esse meio de pesquisa não realiza indexação.

### Nome
Na pesquisa por **Nome**  o algoritmo tenta achar um item que possua a ```&str``` fornecida no campo "nome".

```<Rust>
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
```
Note o padrão: Todas as buscas que realizam indexações vão primeiro tentar buscar o índice .csv no diretório específico. Caso não consigam, eles realizam a pesquisa normalmente. Pode-se notar o algoritmo em ação no trecho a seguir:

```<Rust>
    for produto in catalogo {
        if produto.nome.to_lowercase().contains(&termo.to_lowercase()) {
            resultados.push(produto.clone()); // clone para salvar
        }
    }
```
Para cara ```produto``` em ```catalogo```, ele vai comparar se a propriedade "nome" tem a ```&str``` fornecida na chamada da função.

### Categoria
Na pesquisa por **Categoria**, o sistema funciona de forma similar como visto em **Nome**, sendo um pouco mais rígido.

```<Rust>
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
```
Como visto no trecho:
```<Rust>
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
```
Esse algoritmo é um pouco mais complexo, já que ele lida com a verificação de "Valores menores que" e "Valores maiores que". Para ficar mais claro, no trecho:

```<Rust>
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
```
O sistema faz a mesma verificação para cada ```produto``` em ```catalogo```, sendo elas:
 - se o usuário optou por pesquisar **apenas** valores maiores que "x"
 - se o preço do item realizando o teste é menor (ou maior, a depender da situação) que "x".

Só então o sistema retorna a lista de itens resultantes da pesquisa.

## Desempenho e escalabilidade

Em ambiente controlado, a versão atual do programa foi testada com uma base de dados em .csv com 60 itens (Aproximadamente 5kb) e com uma base de dados massiva, com 5.000.000 itens (Aproximadamente 500mb). A primeira pesquisa pode sempre ser a mais lenta, mas devido a estratégia de indexação do resultado em blocos menores, os retornos posteriores são muito mais rápidos.


## Possíveis melhorias futuras  
  
- Uso de banco de dados relacional (ex: PostgreSQL)  
- Implementação de índices em memória (HashMap)  
- Paralelização das buscas  
- Interface gráfica ou API REST


## Licença  
  
Este projeto está licenciado sob a licença MIT.  
Sinta-se livre para usar, modificar e distribuir este código.
