fn main(){
     
    let nome = "hello";
    let nome2 = String::from("hello world");
 
    let hello = &nome2[6..11]; //slice
    println!("{}", hello);
    println!("{}", nome);
}