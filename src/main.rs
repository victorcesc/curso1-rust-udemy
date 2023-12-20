use std::io;
use std::collections::{HashMap};
mod functions;

enum Direction{
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
enum Gender{
    Male,
    Female
}

#[derive(Debug)]
enum CarType{
    Fiat,
    Ford,
    Renault
}

enum Pagamento{
    Dinheiro(f32),
    CreditoCartao(bool,f32),
    DebitoCartao(bool,f32)
}


fn nacionalidade_carro(car:CarType){
    match car {
        CarType::Fiat => println!("O carro é italiano"),
        CarType::Ford => println!("O carro é americano"),
        CarType::Renault => println!("O carro é frances")
    }

}

fn main() {

    // flow control
    
    // let mut number1 = String::new();
    // io::stdin()
    //     .read_line(&mut number1)
    //     .expect("Erro ao ler");
    
    // let mut number2 = String::new();
    // io::stdin()
    //     .read_line(&mut number2)
    //     .expect("Erro ao ler");
    
    // if functions::convert_to_int(&number1) > functions::convert_to_int(&number2) {
    //     println!("The {} is bigger than {}", number1.trim(), number2.trim());
    // } else {
    //     println!("The {} is smaller than or equal {}", number1.trim(), number2.trim());
    // }


    // ========================= loops ==============================
    // calculate sum of digits
    
    // let mut data_input = String::new();
    // io::stdin()
    //     .read_line(&mut data_input)
    //     .expect("Erro ao ler");

    // let value_i32 = functions::convert_to_int(&data_input);
    // let sum = functions::sum_of_digits(&value_i32);
    
    // println!("Value of sum : {}", sum);

    // let mut entry_fatorial = String::new();
    // io::stdin().read_line(&mut entry_fatorial).expect("Erro");
    // let mut fatorial = 1;
    // let mut entry_int = functions::convert_to_int(&entry_fatorial);    
    // let new: u64 = entry_int as u64;
    // let result = functions::calculate_factorial(new);
    

    // println!("The fatorial of {}! is {}", entry_fatorial.trim(), result);

    // prime numbers
    //let mut n = String::new(); 
    
    // io::stdin()
    //      .read_line(&mut n)
    //      .expect("Erro ao ler");
    
    println!("{}", functions::is_prime(4));
    
    // great common divisor - maximo divisor comum
    let number1 = 15;
    let number2 = 40;
    let test: Vec<i32> = functions::mdc(number1,number2);
    
    println!("Numbers GCD(MDC) : {:?}", test);

    let result = test.iter().fold(1, |acc , &x| acc * x);
    println!("GCD/MDC of  : {:?}", result);



    // TUPLAS

    let tupla = (12, "valores", 3.14, (1,2,3));
    println!("{}", tupla.1);

    // ENUMS
    let player:Direction = Direction::Right;
    match player{
        Direction::Up => println!("O jogador foi pra cima"),
        Direction::Down => println!("O jogador foi pra baixo"),
        Direction::Right => println!("O jogador foi pra direita"),
        Direction::Left => println!("O jogador foi pra esquerda")
    }

    let player_male:Gender = Gender::Male;
    let player_female:Gender = Gender::Female;
    println!("{:?}",player_male);
    println!("{:?}",player_female);
    //let player:Direction = Direction::Left;
    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Ford);
    nacionalidade_carro(CarType::Renault);

    let pessoa_pagamento = Pagamento::DebitoCartao(true,100f32);

    match pessoa_pagamento{
        Pagamento::CreditoCartao(true,f) => println!("A pessoa pagou com Credito valor {}", f),
        Pagamento::DebitoCartao(true,x) => println!("A pessoa pagou com Debito valor {}", x),
        Pagamento::DebitoCartao(false,x) => println!("O pagamento não funcionou com Debito!! Valor : {}",x),
        _ => println!("outro pagamento")
    }

    // code block
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
     
    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }
    let x = Point::new(10,20);
    println!("{:?}",x );

    // shadowing

    let x = 5; // x é uma variável de nível de escopo com o valor 5
     
    {
        let x = x + 1; // x é sombreado aqui com o valor 6
        println!("x inside: {}", x); // imprime "x inside: 6"
    }
     
    println!("x outside: {}", x); // imprime "x outside: 5"


    // references

    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("x is {}", x); 
    


    // strings

   
    let mut minhaString: String = String::from("Ola meu nom eh Henrique");
    println!("O tamanho dessa string eh {},", minhaString.len());
    println!("A minha esta vazia ? {},", minhaString.is_empty());
    for token in minhaString.split_whitespace(){
        println!("{}", token);
    }
    println!("O nome Henrique esta contido an String? {}", minhaString.contains("Henrique"));
    minhaString.push_str("Bem-vindo a aula ");
    println!("{}", minhaString);
     
    let mut frase = String::new();
    frase = "victor,cesconetto".to_string();
    frase = frase.replace(",","" );
    println!("{:?}", frase);


    // arrays

    let numeros_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
    for n in numeros_inteiros.iter(){
        println!("{}", n);
    }
    
    // vectors

    let mut vetores = vec![1, 2, 3, 4];
    vetores.push(5);
    println!("{}", vetores[4]);
    vetores.remove(1);
    println!("{:?}", vetores);
 
    for i in vetores.iter(){
        println!("{}", i);
    }
 
    // hashmaps

    
    let mut hash_map = HashMap::new();
    hash_map.insert("Matematica", 90);
    hash_map.insert("Portugues", 72);
    hash_map.insert("Biologia", 58);
    hash_map.insert("Informatica", 96);
    
    println!("Quantas materias o aluno cursou? {}", hash_map.len());
    
    match hash_map.get("Informatica"){
        Some(k) => println!("O aluno cursou Informatica e tirou {}", k),
        None => println!("O aluno nao cursou Informatica")
    }
    
    
    

}