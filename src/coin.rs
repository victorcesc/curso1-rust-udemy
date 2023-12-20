pub struct Coin{
    value: i32
}

impl Coin{
    pub fn new(val : i32) -> Coin{
        return Coin { value: val };
    }

    pub fn get_value(&self) -> i32{
        return self.value;
    }

    pub fn set_value(&mut self,value : i32){
        self.value = value;
    }

}

