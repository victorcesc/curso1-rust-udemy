struct Pessoa{
    nome: String,
    idade: i32
}

trait Voz{
    fn falar(&self);
    fn tem_voz(&self) -> bool;
}

impl Voz for Pessoa{

    fn falar(&self){
        println!("Olá, meu nome é {}", self.nome);
    }

    fn tem_voz(&self) -> bool {
        if self.idade > 1 {
            return true;
        }
        return false;
        
    }

}

fn main(){
    let p = Pessoa{ 
        nome: String::from("Victor"),
        idade: 42
    };

    println!("Pode {} falar ? {}", p.nome, p.tem_voz());
    

}