// enum FormaGeometrica {
//     Retangulo {base: f64, altura: f64},
//     Quadrado(f64),
//     Triangulo {base: f64, altura: f64, hipotenusa: f64},
//     Circulo {raio: f64},
// }
 
// fn area(forma: &FormaGeometrica) -> f64 {
//     match forma {
//         FormaGeometrica::Retangulo {base, altura} => base * altura,
//         FormaGeometrica::Quadrado(lado) => lado * lado,
//         FormaGeometrica::Triangulo {base, altura, ..} => (base * altura) / 2.0,
//         FormaGeometrica::Circulo {raio} => 3.14159 * raio * raio,
//     }
// }
 
// fn perimetro(forma: &FormaGeometrica) -> f64 {
//     match forma {
//         FormaGeometrica::Retangulo {base, altura} => (base + altura) * 2.0,
//         FormaGeometrica::Quadrado(lado) => lado * 4.0,
//         FormaGeometrica::Triangulo {base, altura, hipotenusa} => base + altura + hipotenusa,
//         FormaGeometrica::Circulo {raio} => 2.0 * 3.14159 * raio,
//     }
// }
 
// fn main() {
//     let forma1 = FormaGeometrica::Triangulo {base: 3.0, altura: 4.0, hipotenusa: 5.0};
//     let forma2 = FormaGeometrica::Retangulo {base: 4.0, altura: 5.0}; 
//     println!("A area da forma 1 é: {}", area(&forma1)); 
//     println!("O perimetro da forma 1 é: {}", perimetro(&forma1)); 
//     println!("A area da forma 2 é: {}", area(&forma2)); 
//     println!("O perimetro da forma 2 é: {}", perimetro(&forma2)); 
// }

// --------------------------------------------------------------------

// struct Pessoa {
//     nome: String,
//     idade: i32,
//     genero: String,
// }
 
// fn main() {
//     let pessoa1 = Pessoa {
//         nome: String::from("Maria"),
//         idade: 25,
//         genero: String::from("Feminino"),
//     };
 
//     let pessoa2 = Pessoa {
//         nome: String::from("João"),
//         idade: 30,
//         genero: String::from("Masculino"),
//     };
 
//     match (pessoa1.idade, pessoa2.idade) {
//         (a, b) if a > b => println!("{} é mais velho que {}", pessoa1.nome, pessoa2.nome),
//         (a, b) if a < b => println!("{} é mais velho que {}", pessoa2.nome, pessoa1.nome),
//         (_, _) => println!("As pessoas têm a mesma idade"),
//     }
// }


// enum Expr {
//     Num(i32),
//     Op(char, Box<Expr>, Box<Expr>),
// }
 
// fn eval(e: Expr) -> i32 {
//     match e {
//         Expr::Num(n) => n,
//         Expr::Op(op, l, r) => {
//             let l = eval(*l);
//             let r = eval(*r);
//             match op {
//                 '+' => l + r,
//                 '-' => l - r,
//                 '*' => l * r,
//                 '/' => l / r,
//                 _ => panic!("Operador inválido"),
//             }
//         }
//     }
// }
 
// fn main() {
//     let expr1 = Expr::Op('+', Box::new(Expr::Num(2)), Box::new(Expr::Num(3)));
//     let expr2 = Expr::Op('*', Box::new(Expr::Num(4)), Box::new(Expr::Num(5)));
//     let expr3 = Expr::Op('-', Box::new(expr1), Box::new(expr2));
//     let expr4 = Expr::Op('/', Box::new(expr3), Box::new(Expr::Num(2)));
    
//     println!("Resultado da expressão é: {}", eval(expr4));
// }


// ----------------------------------------------------

// enum TipoTransacao {
//     Credito,
//     Debito,
// }
 
// struct Transacao {
//     valor: f32,
//     tipo: TipoTransacao,
//     descricao: String,
// }
 
// fn extrato_transacoes(transacoes: Vec<Transacao>) {
//     let mut saldo = 0.0;
//     for transacao in transacoes {
//         match transacao.tipo {
//             TipoTransacao::Credito => {
//                 saldo += transacao.valor;
//                 println!("Transação de crédito realizada no valor de R${:.2} - Saldo atual: R${:.2}", transacao.valor, saldo);
//             },
//             TipoTransacao::Debito => {
//                 if saldo >= transacao.valor {
//                     saldo -= transacao.valor;
//                     println!("Transação de débito realizada no valor de R${:.2} - Saldo atual: R${:.2}", transacao.valor, saldo);
//                 } else {
//                     println!("Saldo insuficiente para realizar transação de débito no valor de R${:.2} - Saldo atual: R${:.2}", transacao.valor, saldo);
//                 }
//             }
//         }
//     }
// }
 
// fn main() {
//     let transacoes = vec![
//         Transacao { valor: 100.0, tipo: TipoTransacao::Credito, descricao: String::from("Salário") },
//         Transacao { valor: 50.0, tipo: TipoTransacao::Debito, descricao: String::from("Compra de comida") },
//         Transacao { valor: 25.0, tipo: TipoTransacao::Debito, descricao: String::from("Compra de roupa") },
//         Transacao { valor: 10.0, tipo: TipoTransacao::Credito, descricao: String::from("Devolução de compra") },
//     ];
 
//     extrato_transacoes(transacoes);
// }


//---------------------------------------------------


struct Produto {
    nome: String,
    preco: f32,
}
 
trait Desconto {
    fn aplicar_desconto(&mut self, porcentagem: f32);
}
 
impl Desconto for Produto {
    fn aplicar_desconto(&mut self, porcentagem: f32) {
        self.preco -= self.preco * (porcentagem / 100.0);
    }
}
 
fn main() {
    let mut produto1 = Produto {
        nome: String::from("Notebook"),
        preco: 2000.0,
    };
    let mut produto2 = Produto {
        nome: String::from("Smartphone"),
        preco: 1500.0,
    };
 
    produto1.aplicar_desconto(10.0);
    produto2.aplicar_desconto(5.0);
 
    println!("O preço do {} é: R$ {:.2}", produto1.nome, produto1.preco);
    println!("O preço do {} é: R$ {:.2}", produto2.nome, produto2.preco);
}