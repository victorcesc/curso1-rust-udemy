pub fn convert_to_int(data_input : & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

pub fn sum_of_digits(value_i32 : & i32) -> i32{
    let mut sum = 0;
    let mut value = *value_i32;// dereference and copy -  solution to get value dereferenced of value_i32
    //** BEFORE */
    /*  while *value_i32 != 0 {
        let mut r = value_i32 % 10;
        sum = sum + r;
        value_i32 = value_i32 / 10;
    }
    */
    // ** AFTER **
    while value != 0 {
        let r = value % 10;
        sum = sum + r;
        value = value / 10;
    }

    sum
}

pub fn calculate_factorial(mut entry_int: u64) -> u64 {
    let mut factorial: u64 = 1;

    while entry_int > 1 {
        factorial *= entry_int;
        entry_int -= 1;
    }

    factorial
}


// mdc - max div comum

pub fn mdc(mut number1: i32, mut number2: i32) -> Vec<i32> {
    println!("1 : {}, 2 : {}", number1,number2);
    let mut i = 2;// starts a prime number
    let mut fact : Vec<i32> = vec![];
    let mut mdc: Vec<i32> = vec![];
    while number1 != 1 || number2 != 1 {        
        if number1 % i == 0 && number2 % i == 0 {
            number1 = number1 / i;
            number2 = number2 / i;
            mdc.push(i);// push to vector the numbers to calculate greatest common divisor (maximo divisor comum)
            fact.push(i);
       
        } else if number1 % i == 0 {
            number1 = number1 / i;
            fact.push(i);
            
        } else if number2 % i == 0 {
            number2 = number2 / i;
            fact.push(i);
        } else {
            i += 1;
        }
    }
    println!("Numbers factored : {:?}", fact);
    mdc
    

}

pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let limiter = (n as f32).sqrt() as i32 + 1;
    for i in 2..limiter {
        if n % i == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

pub fn eh_palindromo(frase : String) -> bool{

    let mut aux = frase.to_lowercase();
    aux = aux.to_lowercase().replace(",", "").replace(" ", "");
    let mut invertido = aux.chars().rev(); // to test the palindrome
    
    for word in aux.chars() {
        let s = invertido.next().unwrap();
        if word != s {
            return false;
        }
    }
    true
    

}