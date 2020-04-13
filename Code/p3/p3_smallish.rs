

pub fn main(){
    let arr = [("grass","bulbasaur", 25), ("grass","tirtwig", 15), ("fire", "charmander", 33), ("fire", "growlithe", 30), ("water", "squirtle", 27), ("water", "tentacruel", 28), ("fire", "groudon", 300)];
    let mut grass = Vec::new();
    let mut fire = Vec::new();
    let mut water = Vec::new();

    for pokemon in arr.iter(){
        match pokemon.0 {
            "grass" => grass.push(pokemon),
            "fire" => fire.push(pokemon),
            "water" => water.push(pokemon),
            _ => println!("not a pokemon"),
        }
    } 

    let mut count = 0;
    loop {
        count += 1;

        if count == 4 {
            break;
        }else if count == 1{
            let mut avg = 0;
            let mut num = 0;
            for pokemon in grass.iter(){
                avg += pokemon.2;
                num += 1;
            }
            println!("Grass average power is {}", (avg/num));
        }else if count == 2{
            let mut avg = 0;
            let mut num = 0;
            for pokemon in fire.iter(){
                avg += pokemon.2;
                num += 1;
            }
            println!("Fire average power is {}", (avg/num));
        }else if count == 3{
            let mut avg = 0;
            let mut num = 0;
            for pokemon in water.iter(){
                avg += pokemon.2;
                num += 1;
            }
            println!("Water average power is {}", (avg/num));
        }
    }
}