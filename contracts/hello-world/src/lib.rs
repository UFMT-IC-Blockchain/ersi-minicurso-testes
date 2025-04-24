#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn calculate_status(env: Env, nota1: i32, nota2: i32, nota3: i32) -> String {
        // Cálculo da média (considerando que as notas são representadas como inteiros com duas casas decimais)
        let media = (nota1 + nota2 + nota3) / 3;

        // Determinação do status
        if media >= 700 { // 7.00 representado como 700
            String::from_str(&env, "Aprovado")
        } else {
            String::from_str(&env, "Reprovado")
        }
    }
}

mod test;