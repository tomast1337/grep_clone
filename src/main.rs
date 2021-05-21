use std::env;
use std::fs;
use std::error;

struct Configuracao {
    comando: String,
    arquivo: String,
    argumento: String,
}

impl Configuracao {
    pub fn executar(&self) {}
}

fn parser(args: &[String]) -> Result<Configuracao, error> {
    if args.len() != 4 {
        Err(e)
    } else {
        let comando: String = args[1].clone();
        let arquivo: String = args[1].clone();
        let argumento: String = args[1].clone();
        OK(Configuracao { comando, arquivo, argumento })
    }
}

fn print_ajuda() {
    println!("/Utilização:\
    grep_clone primeira <arquivo> <palavra> // Encontra e imprime linha com a primeira aparição da palavra no arquivo \
    grep_clone ultima <arquivo> <palavra>   // Encontra e imprime linha com a ultima aparição da palavra no arquivo  \
    grep_clone todas <arquivo> <palavra>    // Encontra e imprime todas as linha com aparição da palavra no arquivo \
    grep_clone ajuda // Imprimir essas infamações ")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parser(&args);
    if config.is_err() {
        print_ajuda()
    } else {
        config.unwrap().executar()
    }
}