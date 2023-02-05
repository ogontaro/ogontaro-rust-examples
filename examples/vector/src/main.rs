struct Pokemon {
    name: String,
    level: u8,
}

fn main() {
    let pokemons = vec![
        Pokemon {
            name: "Pikachu".to_string(),
            level: 5,
        },
        Pokemon {
            name: "Bulbasaur".to_string(),
            level: 10,
        },
        Pokemon {
            name: "Charmander".to_string(),
            level: 15,
        },
        Pokemon {
            name: "Squirtle".to_string(),
            level: 20,
        },
    ];
    let low_level_pokemons = pokemons.iter().filter(|p| p.level < 15);
    for pokemon in low_level_pokemons {
        println!("{} is a low level pokemon", pokemon.name);
    }
    let high_level_pokemons = pokemons.iter().filter(|p| p.level >= 15);
    for pokemon in high_level_pokemons {
        println!("{} is a high level pokemon", pokemon.name);
    }
}
