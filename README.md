# Employee Management System on Soroban

A blockchain-based employee management system built on Soroban, the smart contract platform for Stellar. This system allows organizations to manage employees, track salaries, and handle payments using a custom token.

## Project Overview

The Employee Management System provides the following features:
- Organization setup with admin authentication
- Employee management (add, update, remove employees)
- Employee promotions and rank tracking
- Salary payments using a custom token
- Employee suspension and reactivation
- Bulk salary payments

## Project Structure

```text
.
├── contracts
│   └── management-system
│       ├── src
│       │   ├── contract_import.rs  // Imports the token contract
│       │   ├── contract_states.rs  // Defines data structures and states
│       │   ├── errors.rs           // Error definitions
│       │   ├── lib.rs              // Main entry point
│       │   ├── system.rs           // Core contract functionality
│       │   └── test.rs             // Contract tests
│       ├── Cargo.toml              // Contract dependencies
│       └── Makefile                // Build instructions
├── Cargo.toml                      // Workspace dependencies
└── README.md
```

## Data Structures

### Employee Ranks
The system supports various employee ranks with hierarchical values:
- Intern (1)
- Junior (2)
- Mid (3)
- Senior (4)
- Lead (5)
- Manager (6)
- Director (7)

### Employee Status
Employees can have the following statuses:
- Active
- Suspended

### Employee Data
Each employee record includes:
- Wallet address
- Name
- Salary amount
- Rank
- Status
- Last salary payment timestamp

## Contract Functions

### Administrative Functions
- `initialize`: Set up the organization with admin, name, and token contract
- `update_token_contract`: Change the token contract used for payments

### Employee Management
- `add_employee`: Add a new employee with details
- `remove_employee`: Remove an employee from the system
- `update_employee`: Update employee information (name, salary)
- `promote_employee`: Change an employee's rank and salary
- `suspend_employee`: Temporarily suspend an employee
- `reactivate_employee`: Reactivate a suspended employee

### Payment Functions
- `pay_salary`: Pay salary to a specific employee
- `pay_all_salaries`: Pay salaries to all active employees

### Query Functions
- `get_employee`: Get details for a specific employee
- `get_all_employees`: Get a list of all employees
- `get_institution_info`: Get organization information

## Building and Testing

### Prerequisites
- Rust and Cargo
- Soroban CLI

### Build Instructions
```bash
# Navigate to the management-system contract directory
cd contracts/management-system

# Build the contract
make build
```

### Run Tests
```bash
# Run all tests
make test

# Or using cargo directly
cargo test
```

## Integration with Token Contract

The Employee Management System integrates with a custom token contract (`payme_token.wasm`) for salary payments. This token contract must be deployed separately and its address provided during initialization.

## Security Considerations

- Admin authentication is required for all administrative operations
- Employee data is stored in persistent storage
- Token transfers are handled through the integrated token contract
