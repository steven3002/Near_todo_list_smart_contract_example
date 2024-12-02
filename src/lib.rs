// Import necessary modules from the NEAR SDK
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use std::collections::HashMap;

// Define the structure of a todo item
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TodoItem {
    text: String,
    completed: bool,
}

// Define the main contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TodoList {
    todos: HashMap<AccountId, HashMap<u64, TodoItem>>,
    next_id: u64,
}

// Implement the contract
#[near_bindgen]
impl TodoList {
    #[init]
    pub fn new() -> Self {
        Self {
            todos: HashMap::new(),
            next_id: 0,
        }
    }

    // Add a new todo item
    pub fn add_todo(&mut self, text: String) {
        let account_id = env::signer_account_id();
        let todo = TodoItem {
            text,
            completed: false,
        };

        self.todos
            .entry(account_id.clone())
            .or_insert_with(HashMap::new)
            .insert(self.next_id, todo);
        self.next_id += 1;
    }

    // List all todo items for the current user with their state
    pub fn list_todos(&self) -> Vec<(u64, String, bool)> {
        let account_id = env::signer_account_id();
        if let Some(user_todos) = self.todos.get(&account_id) {
            user_todos
                .iter()
                .map(|(id, todo)| (*id, todo.text.clone(), todo.completed))
                .collect()
        } else {
            vec![]
        }
    }

    // Delete a todo item by its ID for the current user
    pub fn delete_todo(&mut self, id: u64) {
        let account_id = env::signer_account_id();
        if let Some(user_todos) = self.todos.get_mut(&account_id) {
            user_todos.remove(&id);
        } else {
            env::panic_str("Todo item not found");
        }
    }

    // Change the state of a particular todo item to completed for the current user
    pub fn set_completed(&mut self, id: u64) {
        let account_id = env::signer_account_id();
        if let Some(user_todos) = self.todos.get_mut(&account_id) {
            if let Some(todo) = user_todos.get_mut(&id) {
                todo.completed = true;
            } else {
                env::panic_str("Todo item not found");
            }
        } else {
            env::panic_str("Todo item not found");
        }
    }
}

// Unit tests for the contract
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_add_todo() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = TodoList::new();
        contract.add_todo("Test todo".to_string());
        assert_eq!(contract.list_todos().len(), 1);
    }

    #[test]
    fn test_set_completed() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = TodoList::new();
        contract.add_todo("Test todo".to_string());
        contract.set_completed(0);
        let todos = contract.list_todos();
        assert_eq!(todos[0].2, true);
    }

    #[test]
    fn test_delete_todo() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = TodoList::new();
        contract.add_todo("Test todo".to_string());
        contract.delete_todo(0);
        assert_eq!(contract.list_todos().len(), 0);
    }
}
