enum Asset{
    Stocks,
    Bonds,
    Funds,
    Cash
}

impl Asset{
    fn price(&self) -> f64{
        match self {
            Asset::Stocks => 12000.0,
            Asset::Bonds => 30000.0,
            Asset::Funds => 150000.0,
            Asset::Cash => 890.0
        }        
    }
}



fn main(){
    let portfolio : [Asset; 10] = [Asset::Stocks,Asset::Funds,Asset::Bonds,Asset::Cash,Asset::Cash,Asset::Cash,Asset::Bonds,Asset::Funds,Asset::Stocks,Asset::Funds];
    let total: f64 = portfolio.iter().map(|asset| asset.price()).sum();
    println!("{:?}",total);
}