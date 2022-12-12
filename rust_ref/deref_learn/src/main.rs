use std::ops::Deref;

fn main() {
    let obj = Animal{
        type_of : Cat { sound: "Meow".to_string() }
    };

    println!("{:?}", obj.sound())
}

struct Cat{
    sound : String,
}

impl Cat{
    pub fn sound(&self) -> String{
        self.sound.clone()
    }
}

struct Animal{
    type_of : Cat,
}

impl Animal{
    pub fn type_of_(&self) -> Vec<&str>{
        vec!["Fish","Amphibis", "Reptiles","Aves","Mammals"]
    }
}

impl Deref for Animal{
    type Target = Cat;
    fn deref(&self) -> &Self::Target {
        &self.type_of
    }
    
}