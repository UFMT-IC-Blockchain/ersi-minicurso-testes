#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_calculate_status_aprovado() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Notas que resultam em aprovação (7.50, 8.00, 7.00 representadas como 750, 800, 700)
        let status = client.calculate_status(&750, &800, &700);
        assert_eq!(status, String::from_str(&env, "Aprovado"));
    }

    #[test]
    fn test_calculate_status_reprovado() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Notas que resultam em reprovação (5.50, 6.00, 6.25 representadas como 550, 600, 625)
        let status = client.calculate_status(&550, &600, &625);
        assert_eq!(status, String::from_str(&env, "Reprovado"));
    }

    #[test]
    fn test_calculate_status_limite_aprovado() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Média exatamente no limite de aprovação (7.00 representado como 700)
        let status = client.calculate_status(&700, &700, &700);
        assert_eq!(status, String::from_str(&env, "Aprovado"));
    }

    #[test]
    fn test_calculate_status_limite_reprovado() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Média ligeiramente abaixo do limite de aprovação (6.99 representado como 699)
        let status = client.calculate_status(&699, &699, &699);
        assert_eq!(status, String::from_str(&env, "Reprovado"));
    }

    #[test]
    fn test_calculate_status_notas_altas() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Notas altas para verificar comportamento (10.00, 9.50, 9.00 representadas como 1000, 950, 900)
        let status = client.calculate_status(&1000, &950, &900);
        assert_eq!(status, String::from_str(&env, "Aprovado"));
    }
}