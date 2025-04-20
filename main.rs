struct Fruit {
    apples: i32,
    bananas: i32,
}

fn increase_fruit( fruit: &mut Fruit) {
    fruit.apples *= 2;
    fruit.bananas *= 3;
    

}

fn print_fruit(fruit: &Fruit) {
    println!("You have {} apples and {} bananas", fruit.apples, fruit.bananas);
    
}

fn main() {
    let mut fruit = Fruit {
        apples: 10,
        bananas: 5,

    };

    print_fruit(&fruit);
    increase_fruit(&mut fruit);
    
}