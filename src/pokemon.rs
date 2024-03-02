use lazy_static::lazy_static;
use rand::prelude::*;

static POKEDEX: &str = include_str!("../pokedex");

lazy_static! {
    static ref POKEMON_LIST: Vec<&'static str> = POKEDEX.split('\n').collect();
}

pub fn random_name() -> String {
    POKEMON_LIST.get(thread_rng().gen_range(0..POKEMON_LIST.len()))
        .expect("should not overflow").to_string()
}