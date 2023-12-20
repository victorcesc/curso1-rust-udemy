mod coin;

use coin::Coin;

fn main(){
    let mut coin = Coin::new(35);
    println!("Value of coin {}", coin.get_value());
    coin.set_value(3);
    println!("Value of coin after changes {}", coin.get_value());
}