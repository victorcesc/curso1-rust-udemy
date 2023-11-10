struct User{
    username: String,
    email: String,
    ativo: bool,
    genero: String,
}

fn user(usuario: &User){
    println!("O nome do usuario eh {}", usuario.username);
}


fn main(){
    let mut pessoa = User {username:String::from("JoaoPessoa"), email:String::from("joaoPessoa@gmail"),ativo:true, genero:String::from("Homem")};
    pessoa.ativo = false;
    println!("O nome do usuario eh {}, seu email eh {} e seu genero eh {}", pessoa.username, pessoa.email, pessoa.genero);
    user(&pessoa);
    user(&pessoa);
    
}



