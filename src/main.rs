use std::env;
use std::fs;
use ansi_term::Colour;

struct Configuracao {
    comando: String,
    arquivo: String,
    argumento: String,
}

impl Configuracao {
    pub fn executar(&self) {
        match self.comando.as_str() {
            "primeira" => primeiro(&self.arquivo, &self.argumento),
            "ultima" => ultima(&self.arquivo, &self.argumento),
            "todas" => todas(&self.arquivo, &self.argumento),
            "ajuda" => print_ajuda(),
            _ => panic!("Erro nos argumentos"),
        }
    }
}

fn primeiro(arquivo: &String, argumento: &String) {
    let conteudo = get_linhas_conteudo(&arquivo);
    for linha in conteudo {
        if linha.contains(&argumento.as_str()) {
            println!("{}", linha.replace(argumento, &Colour::Red.paint(argumento)));
            break;
        }
    }
}

fn ultima(arquivo: &String, argumento: &String) {
    let conteudo = get_linhas_conteudo(arquivo);
    for linha in conteudo.iter().rev() {
        if linha.contains(&argumento.as_str()) {
            println!("{}", linha.replace(argumento, &Colour::Red.paint(argumento)));
            break;
        }
    }
}

fn todas(arquivo: &String, argumento: &String) {
    let conteudo = get_linhas_conteudo(arquivo);
    for linha in conteudo {
        if linha.contains(&argumento.as_str()) {
            println!("{}", linha.replace(argumento, &Colour::Red.paint(argumento)));
        }
    }
}

fn get_linhas_conteudo(arquivo: &String) -> Vec<String> {
    fs::read_to_string(arquivo).expect("Erro ao abrir o arquivo").split("\n").map(|x| String::from(x)).collect()
}

fn parser(args: &[String]) -> Configuracao {
    let comando: String;
    let arquivo: String;
    let argumento: String;
    if args.len() < 4 {
        if args.len() == 2 && args[1] == "ajuda" {
            comando = args[1].clone();
            arquivo = String::new();
            argumento = String::new();
            Configuracao { comando, arquivo, argumento }
        } else {
            panic!("Erro nos parâmetros utilize grep_clone ajuda para ver como se utiliza o programa ")
        }
    } else {
        comando = args[1].clone();
        arquivo = args[2].clone();
        argumento = args[3].clone();
        match comando.as_str() {
            "primeira" | "ultima" | "todas" | "ajuda" => (),
            _ => panic!("Erro no primeiro parâmetro"),
        }
        println!("{},{},{}", comando, arquivo, argumento);
        Configuracao { comando, arquivo, argumento }
    }
}

fn print_ajuda() {
    println!("Utilização: \n \
    grep_clone primeira <arquivo> <palavra> // Encontra e imprime linha com a primeira aparição da palavra no arquivo \n \
    grep_clone ultima <arquivo> <palavra>   // Encontra e imprime linha com a ultima aparição da palavra no arquivo  \n \
    grep_clone todas <arquivo> <palavra>    // Encontra e imprime todas as linha com aparição da palavra no arquivo \n \
    grep_clone ajuda // Imprimir essas infamações ")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parser(&args);
    config.executar()
}