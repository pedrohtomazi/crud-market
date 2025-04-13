#[derive(serde::Serialize, serde::Deserialize)]
struct Produto {
    nome: String,
    preco: i32,
}

fn carregar_produtos() -> Vec<Produto> {
    let dados = std::fs::read_to_string("produtos.json").expect("Erro ao ler arquivo");
    let dados_lidos: Vec<Produto> = serde_json::from_str(&dados).expect("erro ao converter");
    dados_lidos
}

fn salvar_produtos(produtos: &Vec<Produto>) {
    let json = serde_json::to_string_pretty(produtos).expect("Erro ao converter para JSON");
    std::fs::write("produtos.json", json).expect("Erro ao salvar o arquivo");
}

fn alterar_preco(produtos: &mut Vec<Produto>) {
    use std::io::{self, Write};

    print!("Digite o nome do produto: ");
    io::stdout().flush().unwrap();
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();

    print!("Digite o novo preço: ");
    io::stdout().flush().unwrap();
    let mut preco_str = String::new();
    io::stdin().read_line(&mut preco_str).unwrap();
    let preco: f32 = preco_str.trim().parse().expect("Preço inválido");

    for produto in produtos.iter_mut() {
        if produto.nome == nome {
            produto.preco = preco as i32;
            println!("Preço do produto '{}' alterado para {:.2}", nome, preco);
            return;
        }
    }

    println!("Produto '{}' não encontrado!", nome);
}

fn remover_produto(produtos: &mut Vec<Produto>) {
    use std::io::{self, Write};

    print!("Digite o nome do produto que deseja remover: ");
    io::stdout().flush().unwrap();

    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();

    let antes = produtos.len();
    produtos.retain(|p| p.nome != nome);

    if produtos.len() < antes {
        println!("Produto '{}' removido com sucesso!", nome);
    } else {
        println!("Produto '{}' não encontrado.", nome);
    }
}

fn menu(dados_lidos: &Vec<Produto>) {
    'principal: loop {   
    
    println!("Bem vindo ao menu! \n Qual opção você deseja escolher");
    println!("Para cadastrar produtos digite: 1");
    println!("Para visualizar produtos digite: 2");
    println!("Para alterar preço dos produtos digite: 3");
    println!("Para remover produtos digite: 4");
    println!("Para finalizar digite 5");

    let mut optionmenu = String::new();
    std::io::stdin().read_line(&mut optionmenu).expect("Opção invalida");
    let select:i32 = optionmenu.trim().parse().expect("Erro ao converter o input");
    
    if select == 1 { //OPÇÃO PRO USUARIO CADASTRAR PRODUTOS
        'cadastro: loop {
        println!("Digite o nome do seu produto:");
        let mut produto_push = String::new();
        std::io::stdin().read_line(&mut produto_push).expect("Opção invalida");
        println!("Digite o preço do seu produto:");
        let mut price_push = String::new();
        std::io::stdin().read_line(&mut price_push).expect("Opção invalida");
        let price_push: i32 = price_push.trim().parse().expect("Erro ao converter o preço");

        let produto_final_push = Produto {
            nome: produto_push.trim().to_string(),
            preco: price_push,
        };
        println!("{} / {}", produto_final_push.nome, produto_final_push.preco);


        println!("Deseja salvar esse produto?\nSe sim: digite: 1\nSe desejar voltar ao menu sem salvar digite: 2");

        let mut option_save_push = String::new();
        std::io::stdin().read_line(&mut option_save_push).expect("Opção invalida");
        let option_save_push:i32 = option_save_push.trim().parse().expect("Erro ao converter o input");

        if option_save_push == 1 {
            let mut produtos = carregar_produtos(); // carrega os já existentes
            produtos.push(produto_final_push);      // adiciona o novo
            salvar_produtos(&produtos);             // salva todos
            println!("Produto Salvo");
             
            loop {
            println!("Deseja salvar mais produtos?\nSe sim: digite: 1\nSe desejar voltar ao menu digite: 2");

            let mut option_save_and_create = String::new();
            std::io::stdin().read_line(&mut option_save_and_create).expect("Opção invalida");
            let option_save_and_create:i32 = option_save_and_create.trim().parse().expect("Erro ao converter o input");
                if option_save_and_create == 1 {
                    break 'cadastro;
                }
                if option_save_and_create == 2 {
                    continue 'principal;
                } else {
                    println!("Opção invalida");
                    continue;
                }
            }
                
        } else if option_save_push == 2 {
            continue 'principal;
        } 
        }
    }
    if select == 2 { //OPÇÃO PRO USUARIO VISUALIZAR PRODUTOS

        for produto in dados_lidos {
            println!("{}, {}", produto.nome, produto.preco);
        }
        loop {
        println!("Para voltar ao Menu digite 1:");
        let mut option_ler_dados = String::new();
        std::io::stdin().read_line(&mut option_ler_dados).expect("Opção invalida");
        let option_ler_dados:i32 = option_ler_dados.trim().parse().expect("Erro ao converter o input");
        if option_ler_dados == 1 {
            break 'principal;
        } else {
            println!("Opção Invalida:");
            continue;
        }
    }
    }
    if select == 3 { //OPÇÃO PRO USUARIO ALTERAR PRECO
        if select == 3 {
            let mut produtos = carregar_produtos();
            alterar_preco(&mut produtos);
            salvar_produtos(&produtos);
        }


    }
    if select == 4 { //OPÇÃO PRO USUARIO REMOVER PRODUTOS
        carregar_produtos();
        let mut produtos = carregar_produtos();
        remover_produto(&mut produtos);
        salvar_produtos(&produtos);
    }
    if select == 5 { //OPÇÃO PRO USUARIO FECHAR O PROGRAMA
        use std::{thread, time};
        println!("Encerrando...");
        thread::sleep(time::Duration::from_secs(5));
        std::process::exit(0);
    }
}
}
fn main() {
    let dados_lidos = carregar_produtos();
    menu(&dados_lidos);
}