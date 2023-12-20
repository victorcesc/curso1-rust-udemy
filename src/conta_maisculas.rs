fn conta_maiusculas(palavra: String) -> i32{
    let mut contador : i32 = 0;
    
    for word in palavra.chars(){
        if word.is_uppercase(){
            contador += 1
        }
    }
    contador
}


fn main(){

    let frase = "Victor Cesconetto De Pieri".to_string();
    println!("{:?}", conta_maiusculas(frase));


}