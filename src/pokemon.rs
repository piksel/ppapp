use lazy_static::lazy_static;
use rand::prelude::*;

static POKEDEX: &str = include_str!("../pokedex");

lazy_static! {
    static ref pokemon_list: Vec<&'static str> = POKEDEX.split('\n').collect();
}

pub fn random_name() -> String {
    pokemon_list.get(thread_rng().gen_range(0..pokemon_list.len()))
        .expect("should not overflow").to_string()
}