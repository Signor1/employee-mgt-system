#![cfg(test)]
use crate::contract_import::token_contract::Client as TokenContractClient;
use crate::contract_states::*;
use crate::errors::ContractError;
use crate::system::{EmployeeManagementContract, EmployeeManagementContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn create_token_contract(env: &Env, admin: &Address) -> Address {
    let name = String::from_str(env, "PaymeToken");
    let symbol = String::from_str(env, "PAY");
    let decimals = 18u32;

    let token_contract_id = env.register(
        crate::contract_import::token_contract::WASM,
        (admin, name.clone(), symbol.clone(), decimals),
    );

    token_contract_id
}

fn setup_test() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let employee2 = Address::generate(&env);
    let token_contract = create_token_contract(&env, &admin);

    (env, admin, employee1, employee2, token_contract)
}

#[test]
fn test_initialization() {
    let (env, admin, _, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());

    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin,
        &String::from_str(&env, "Test Institution"),
        &token_contract,
    );

    assert_eq!(result, ());

    // Test that we can get institution info
    let institution_info = mgt_client.get_institution_info();
    assert_eq!(institution_info.admin, admin);
    assert_eq!(
        institution_info.name,
        String::from_str(&env, "Test Institution")
    );
    assert_eq!(institution_info.total_employees, 0_u32);
    assert_eq!(institution_info.token_contract, token_contract);

    let token_client = TokenContractClient::new(&env, &token_contract);
    assert_eq!(token_client.name(), String::from_str(&env, "PaymeToken"));
    assert_eq!(token_client.symbol(), String::from_str(&env, "PAY"));
    assert_eq!(token_client.decimals(), 18u32);
}
