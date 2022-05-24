use std::collections::{HashMap, VecDeque};
use crate::coroutine::{Coroutine, CoroutineState};

#[derive(Clone, Debug)]
pub(crate) enum ByteCode {
    // loads the value to the stack
    LoadVal { value: i128 },
    // Reading a variable from storage
    ReadVar { name: String },
    // Writing a variable to the storage
    WriteVar { name: String },
    // Takes two numbers from the stack, adds them up, and puts the result back
    Add,
    // Takes two numbers from the stack, subtracts the first from the second and returns the result back
    Minus,
    // Takes two numbers from the stack, multiplies them, and puts the result back
    Multiply,
    // Takes two numbers from the stack, division them, and puts the result back
    Division,
    // Shuts down the coroutine
    Return,
    // Shuts down the coroutine, and prints the result
    ReturnValue,
    // Jumps to the bytecode address
    Jump { address_instruction: usize },
    // Jumps to the bytecode address if the last value on the stack is greater than 0
    JumpIfHeadStackTrue { address_instruction: usize },
    // Checks that the channel is empty, and if it is not empty, it is blocked. Then sends the value.
    SendChannel { name: String, value: i128 },
    // Checks that the channel exists, and if not, it is blocked. Then it reads the value.
    RecvChannel { name: String },
    // Takes the last two values from the stack, and launches two new coroutine,
    // which start execution from addresses taken from the stack
    Spawn,
    // Logs the stack head
    Log,
    // Takes and logs the stack head
    PopLog,
}

impl ByteCode {
    pub(crate) fn execute(&self, coroutines: &mut Vec<Coroutine>, coroutine: &mut Coroutine,
                          store: &mut HashMap<String, i128>) -> CoroutineState {
        match self {
            ByteCode::LoadVal { .. } |
            ByteCode::ReadVar { .. } |
            ByteCode::WriteVar { .. } |
            ByteCode::Add |
            ByteCode::Minus |
            ByteCode::Multiply |
            ByteCode::Division |
            ByteCode::Return |
            ByteCode::ReturnValue |
            ByteCode::Jump { .. } |
            ByteCode::Spawn |
            ByteCode::Log |
            ByteCode::PopLog => {
                match self {
                    ByteCode::LoadVal { value } =>
                        ByteCode::load_val(&mut coroutine.stack, value.clone()),
                    ByteCode::ReadVar { name } =>
                        ByteCode::read_var(store, &mut coroutine.stack, name),
                    ByteCode::WriteVar { name } =>
                        ByteCode::write_var(store, &mut coroutine.stack, name),
                    ByteCode::Add => ByteCode::add(&mut coroutine.stack),
                    ByteCode::Minus => ByteCode::minus(&mut coroutine.stack),
                    ByteCode::Multiply => ByteCode::multiply(&mut coroutine.stack),
                    ByteCode::Division => ByteCode::division(&mut coroutine.stack),
                    ByteCode::Return => {}
                    ByteCode::ReturnValue => ByteCode::return_value(&mut coroutine.stack),
                    ByteCode::Jump { .. } => {}
                    ByteCode::SendChannel { .. } => {}
                    ByteCode::RecvChannel { .. } => {}
                    ByteCode::Spawn => ByteCode::spawn(&mut coroutine.stack, coroutines),
                    ByteCode::Log => ByteCode::log(&mut coroutine.stack),
                    ByteCode::PopLog => ByteCode::pop_log(&mut coroutine.stack),
                    _ => {}
                }
                coroutine.increment_address_current_instruction(self, false);
                match self {
                    ByteCode::Return | ByteCode::ReturnValue => CoroutineState::Fulfilled,
                    _ => CoroutineState::Pending,
                }
            }
            ByteCode::JumpIfHeadStackTrue { address_instruction } => {
                let value = coroutine.stack.pop_back().unwrap();
                if value > 0 {
                    coroutine.increment_address_current_instruction(
                        &ByteCode::Jump { address_instruction: address_instruction.clone() },
                        false,
                    );
                } else {
                    coroutine.increment_address_current_instruction(self, false);
                }
                CoroutineState::Pending
            }
            ByteCode::SendChannel { name, value } => {
                let key = format!("__channel_{}", name);
                if store.contains_key(&key) {
                    coroutine.increment_address_current_instruction(self,
                                                                    true);
                    return CoroutineState::Pending;
                }

                store.insert(key, value.clone());

                coroutine.increment_address_current_instruction(self, false);
                CoroutineState::Pending
            }
            ByteCode::RecvChannel { name } => {
                let key = format!("__channel_{}", name);
                if !store.contains_key(&key) {
                    coroutine.increment_address_current_instruction(self,
                                                                    true);
                    return CoroutineState::Pending;
                }

                coroutine.stack.push_back(store.get(&key).unwrap().clone());
                store.remove(&key);

                coroutine.increment_address_current_instruction(self, false);
                CoroutineState::Pending
            }
        }
    }

    fn load_val(stack: &mut VecDeque<i128>, value: i128) {
        stack.push_back(value);
    }

    fn read_var(store: &mut HashMap<String, i128>,
                stack: &mut VecDeque<i128>, name: &String) {
        let value = store.get(name).unwrap().clone();
        stack.push_back(value);
    }

    fn write_var(store: &mut HashMap<String, i128>,
                 stack: &mut VecDeque<i128>, name: &String) {
        let value = stack.pop_back().unwrap();
        store.insert(name.clone(), value);
    }

    fn add(stack: &mut VecDeque<i128>) {
        let add = |a, b| a + b;
        ByteCode::lift_binary_operation(stack, add);
    }

    fn minus(stack: &mut VecDeque<i128>) {
        let minus = |a, b| a - b;
        ByteCode::lift_binary_operation(stack, minus);
    }

    fn multiply(stack: &mut VecDeque<i128>) {
        let multiply = |a, b| a * b;
        ByteCode::lift_binary_operation(stack, multiply);
    }

    fn division(stack: &mut VecDeque<i128>) {
        let division = |a, b| a / b;
        ByteCode::lift_binary_operation(stack, division);
    }

    fn lift_binary_operation(stack: &mut VecDeque<i128>,
                             func: fn(i128, i128) -> i128) {
        let b = stack.pop_back().unwrap();
        let a = stack.pop_back().unwrap();
        let value = func(a, b);
        stack.push_back(value);
    }

    fn return_value(stack: &mut VecDeque<i128>) {
        println!("{:?}", stack.pop_back());
    }

    fn spawn(stack: &mut VecDeque<i128>, coroutines: &mut Vec<Coroutine>) {
        let address_instruction_b = stack.pop_back().unwrap();
        let address_instruction_a = stack.pop_back().unwrap();
        coroutines.push(Coroutine::new(
            address_instruction_a as usize
        ));
        coroutines.push(Coroutine::new(
            address_instruction_b as usize
        ));
    }

    fn log(stack: &mut VecDeque<i128>) {
        let value = stack.pop_back().unwrap();
        println!("{:?}", value);
        stack.push_back(value);
    }

    fn pop_log(stack: &mut VecDeque<i128>) {
        let value = stack.pop_back();
        println!("{:?}", value);
    }
}