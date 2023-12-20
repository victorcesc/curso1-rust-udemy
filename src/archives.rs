use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::io;
 
fn adicionar_linhas_ao_final(arquivo_path: &str, linhas: Vec<&str>) -> io::Result<()> {
    // Abre o arquivo no modo de escrita (append)
    let mut arquivo = fs::OpenOptions::new().create(true).append(true).open(arquivo_path)?;
    let arquivo_existe = fs::metadata(arquivo_path).is_ok();
    
    
    // Adiciona as linhas ao final do arquivo
    for linha in linhas {
        writeln!(arquivo, "{}", linha)?;
    }

    Ok(())
}

fn main(){
    let arquivo_path = "/home/victorcesc/Projetos/rust-general/curso-udemy/curso/files/dados.txt";

    let arquivo_existe = fs::metadata(arquivo_path).is_ok();

    if arquivo_existe {
        let mut arquivo = File::open(arquivo_path).expect("Nao conseguiu ler");
        let mut conteudo = String::new();
        arquivo.read_to_string(&mut conteudo).expect("Nao conseguiu ler o arquivo e alocar na variavel conteudo");
        println!("O conteudo em arquivo eh :\n\n{}", conteudo);
        //numero de linhas 
        let n_lines = conteudo.lines().count();
        println!("numero de linhas : {}", n_lines);
        //adicionando novas linhas
        let linhas : Vec<&str> = vec!["Linha1", "Linha2"];
        match adicionar_linhas_ao_final(arquivo_path, linhas) {
            Ok(_) => println!("Linhas adicionadas com sucesso."),
            Err(e) => eprintln!("Erro ao adicionar linhas: {}", e),
        }
    } else {
        //cria arquivo
        let mut c_arquivo = File::create(arquivo_path)
            .expect("Não conseguiu criar o arquivo");
        c_arquivo.write_all(b"Victor\nCesconetto\n")
            .expect("Não conseguiu escrever no arquivo");
    }
    
    
    
}


