use std::io;
mod functions;

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

}