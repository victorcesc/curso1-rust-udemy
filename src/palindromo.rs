fn eh_palindromo(frase : String) -> bool{

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


fn main(){

    let mut frase = String::new();
    frase = "Rir, o breve verbo rir".to_string();
    println!("{:?}", eh_palindromo(frase));


}