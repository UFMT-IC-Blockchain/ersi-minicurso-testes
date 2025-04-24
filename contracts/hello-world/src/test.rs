#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_calculate_status_aprovado() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let status = client.calculate_status(&750, &800, &700);
    assert_eq!(status, String::from_str(&env, "Aprovado"));
}

#[test]
fn test_calculate_status_reprovado() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let status = client.calculate_status(&550, &600, &625);
    assert_eq!(status, String::from_str(&env, "Reprovado"));
}

#[test]
fn test_calculate_status_limite_aprovado() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let status = client.calculate_status(&700, &700, &700);
    assert_eq!(status, String::from_str(&env, "Aprovado"));
}

#[test]
fn test_calculate_status_limite_reprovado() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let status = client.calculate_status(&699, &699, &699);
    assert_eq!(status, String::from_str(&env, "Reprovado"));
}

#[test]
fn test_calculate_status_notas_altas() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let status = client.calculate_status(&1000, &950, &900);
    assert_eq!(status, String::from_str(&env, "Aprovado"));
}
