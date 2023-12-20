struct Produto{
    nome : String,
    preco : f64,
    qtd : u32
}


impl Produto{
    pub fn new(n: String, p: f64, q: u32) -> Produto{
        return Produto { nome: n , preco: p, qtd: q }
    }
    pub fn set_nome(&mut self, nome : String){
        self.nome = nome;
    }
    pub fn set_preco(&mut self, preco : f64){
        self.preco = preco;
    }
    pub fn set_qtd(&mut self, qtd : u32){
        self.qtd = qtd;
    }
    pub fn produto_to_string(&self) -> String{
        format!("Nome do produto : {}\nPreco do produto : {}\nQuantidade do produto : {}", self.nome, self.preco, self.qtd)
    }

}

fn main(){
    let mut prod = Produto::new("Banana".to_string(),0.65,22);
   
    println!("{}", prod.produto_to_string());
    prod.set_nome("Maçã".to_string());
    prod.set_preco(0.88);
    prod.set_qtd(35);
    println!("{}", prod.produto_to_string());
}