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

#[test]
fn test_initialization_already_initialized() {
    let (env, admin, _, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    // First initialization should succeed
    mgt_client.initialize(
        &admin,
        &String::from_str(&env, "Test Institution"),
        &token_contract,
    );

    // Second initialization should panic
    let result = mgt_client.try_initialize(
        &admin,
        &String::from_str(&env, "Test Institution"),
        &token_contract,
    );

    assert_eq!(
        result.unwrap_err(),
        Ok(ContractError::AlreadyInitialized),
        "Error should be AlreadyInitialized"
    );
}

#[test]
fn test_add_employee() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Add employee
    let result = mgt_client.add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, "John Doe"),
        &1000,
        &EmployeeRank::Junior,
    );

    assert_eq!(result, ());

    // Check employee was added
    let employee = mgt_client.get_employee(&employee1);
    assert_eq!(employee.name, String::from_str(&env, "John Doe"));
    assert_eq!(employee.salary, 1000);
    assert_eq!(employee.rank, EmployeeRank::Junior);
    assert_eq!(employee.status, EmployeeStatus::Active);

    // Check institution total employees updated
    let institution_info = mgt_client.get_institution_info();
    assert_eq!(institution_info.total_employees, 1);
}

#[test]
fn test_add_employee_invalid_salary() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Add employee
    let result = mgt_client.try_add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, "John Doe"),
        &0,
        &EmployeeRank::Junior,
    );

    assert_eq!(
        result.unwrap_err(),
        Ok(ContractError::InvalidSalary),
        "Error should be InvalidSalary"
    );
}

#[test]
fn test_add_employee_empty_name() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "Quality Assurance Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Add employee
    let result = mgt_client.try_add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, ""),
        &1000,
        &EmployeeRank::Junior,
    );

    assert_eq!(
        result.unwrap_err(),
        Ok(ContractError::InvalidName),
        "Error should be InvalidName"
    );
}

#[test]
fn test_add_employee_already_exists() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Add first employee
    mgt_client.add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, "John Doe"),
        &1000,
        &EmployeeRank::Junior,
    );

    // Add same employee
    let result = mgt_client.try_add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, "John Doe"),
        &1000,
        &EmployeeRank::Junior,
    );

    assert_eq!(
        result.unwrap_err(),
        Ok(ContractError::EmployeeAlreadyExists),
        "Error should be EmployeeAlreadyExists"
    );
}

#[test]
fn test_remove_employee() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Add first employee
    mgt_client.add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, "John Doe"),
        &1000,
        &EmployeeRank::Junior,
    );

    // Remove employee
    let result = mgt_client.remove_employee(&admin, &employee1);

    assert_eq!(result, ());

    let result = mgt_client.try_get_employee(&employee1);

    assert_eq!(
        result.unwrap_err(),
        Ok(ContractError::EmployeeNotFound),
        "Error should be EmployeeNotFound"
    );

    let institution_info = mgt_client.get_institution_info();

    assert_eq!(institution_info.total_employees, 0);
}

#[test]
fn test_remove_employee_not_found() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Remove employee
    let result = mgt_client.try_remove_employee(&admin, &employee1);

    assert_eq!(
        result.unwrap_err(),
        Ok(ContractError::EmployeeNotFound),
        "Error should be EmployeeNotFound"
    );

    let institution_info = mgt_client.get_institution_info();

    assert_eq!(institution_info.total_employees, 0);
}

#[test]
fn test_update_employee() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Add first employee
    mgt_client.add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, "John Doe"),
        &1000,
        &EmployeeRank::Junior,
    );

    // Update employee
    let result = mgt_client.update_employee(
        &admin,
        &employee1,
        &Some(String::from_str(&env, "Jane Linda")),
        &Some(2000),
    );

    assert_eq!(result, ());

    let employee_info = mgt_client.get_employee(&employee1);

    assert_eq!(employee_info.name, String::from_str(&env, "Jane Linda"));
    assert_eq!(employee_info.salary, 2000);

    let institution_info = mgt_client.get_institution_info();

    assert_eq!(institution_info.total_employees, 1);
}

#[test]
fn test_update_employee_not_found() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Update employee
    let result = mgt_client.try_update_employee(
        &admin,
        &employee1,
        &Some(String::from_str(&env, "Jane Linda")),
        &Some(2000),
    );

    assert_eq!(
        result.unwrap_err(),
        Ok(ContractError::EmployeeNotFound),
        "Error should be EmployeeNotFound"
    );

    let institution_info = mgt_client.get_institution_info();

    assert_eq!(institution_info.total_employees, 0);
}

#[test]
fn test_promote_employee() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Add first employee
    mgt_client.add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, "John Doe"),
        &1000,
        &EmployeeRank::Junior,
    );

    let result = mgt_client.promote_employee(&admin, &employee1, &EmployeeRank::Senior, &2000);

    assert_eq!(result, ());

    let employee_info = mgt_client.get_employee(&employee1);

    assert_eq!(employee_info.rank, EmployeeRank::Senior);
    assert_eq!(employee_info.salary, 2000);
}

#[test]
fn test_promote_employee_same_rank() {
    let (env, admin, employee1, _, token_contract) = setup_test();
    let contract_id = env.register(EmployeeManagementContract, ());
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "QA Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Add first employee
    mgt_client.add_employee(
        &admin,
        &employee1,
        &String::from_str(&env, "John Doe"),
        &1000,
        &EmployeeRank::Junior,
    );

    let result = mgt_client.try_promote_employee(&admin, &employee1, &EmployeeRank::Junior, &1000);

    assert_eq!(
        result.unwrap_err(),
        Ok(ContractError::SameRank),
        "Error: Same rank"
    );

    let employee_info = mgt_client.get_employee(&employee1);

    assert_eq!(employee_info.rank, EmployeeRank::Junior);
    assert_eq!(employee_info.salary, 1000);
}
