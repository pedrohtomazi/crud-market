# crud-market
 
# 📦 Sistema de Gerenciamento de Produtos em Rust

Este projeto é um **CRUD de produtos em Rust**, com armazenamento em JSON. A aplicação roda no terminal, oferecendo um menu interativo para o usuário gerenciar produtos.

---

## 🚀 Funcionalidades

- ✅ Cadastrar novos produtos
- 📄 Listar produtos já cadastrados
- ✏️ Alterar o preço de um produto existente
- ❌ Remover um produto
- 💾 Persistência dos dados no arquivo `produtos.json`

---

## 🧠 Objetivo

Este projeto foi desenvolvido como parte do aprendizado da linguagem **Rust**, com foco em:
- Entrada e saída de dados
- Manipulação de arquivos (`std::fs`)
- Serialização com `serde`
- Estruturação e controle de fluxo

---

## 📁 Estrutura do Projeto

### Structs

#### `Produto`
#[derive(serde::Serialize, serde::Deserialize)]
struct Produto {
    nome: String,
    preco: i32,
}
Representa um produto com nome e preço. É serializável com serde para poder ler/gravar em JSON.

###### 🔧 Funções do Projeto
carregar_produtos() -> Vec<Produto>
Lê o arquivo produtos.json e retorna um Vec<Produto> com os dados.
Se o arquivo não existir ou der erro, o programa irá "panic" com uma mensagem de erro.

salvar_produtos(produtos: &Vec<Produto>)
Recebe um vetor de produtos e salva ele em formato JSON no arquivo produtos.json, com indentação (pretty).

alterar_preco(produtos: &mut Vec<Produto>)
Pergunta o nome de um produto e o novo preço via terminal.
Se o produto for encontrado, atualiza o preço.
Se não for encontrado, informa o usuário.

remover_produto(produtos: &mut Vec<Produto>)
Pergunta o nome de um produto.
Remove o produto do vetor se ele for encontrado.
Mostra feedback de sucesso ou erro.

menu(dados_lidos: &Vec<Produto>)
Função principal que exibe o menu para o usuário com as opções:

Cadastrar produto

Listar produtos

Alterar preço

Remover produto

Finalizar o programa

Cada opção chama as funções adequadas.
Possui loops e controle de fluxo para retornar ao menu ou continuar conforme o usuário desejar.

main()
Função de entrada do programa.
Carrega os produtos existentes e chama o menu.

## 🧪 Exemplo de Uso
Bem vindo ao menu!
Qual opção você deseja escolher
1 - Cadastrar produto
2 - Visualizar produtos
3 - Alterar preço
4 - Remover produto
5 - Finalizar
## 📦 Dependências
Adicione no seu Cargo.toml:


## [dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
💻 Como Executar
Clone o projeto:


git clone https://github.com/seu-usuario/nome-do-repo.git
cd nome-do-repo
Compile:

cargo build
Rode:


cargo run
## 📝 Observações Técnicas
O campo preco é salvo como i32, ou seja, não suporta centavos.
Se quiser usar valores com casas decimais, altere para f64.

O uso de unwrap e expect torna o código mais simples, mas menos robusto.
Para uso em produção, substitua por match ou if let para evitar panics.

## ✍️ Autor
Pedro Tomazi
Programador Rust, entusiasta de automação e projetos educacionais com foco em produtividade.
