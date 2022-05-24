use std::collections::{HashMap, VecDeque};
use crate::ByteCode;

#[derive(Clone)]
pub(crate) struct Coroutine {
    address_current_instruction: usize,
    pub(crate) stack: VecDeque<i128>,
}

impl Coroutine {
    pub(crate) fn new(address_instruction: usize) -> Coroutine {
        return Coroutine {
            address_current_instruction: address_instruction,
            stack: Default::default(),
        };
    }

    pub(crate) fn execute_byte_code(&mut self, coroutines: &mut Vec<Coroutine>,
                                    store: &mut HashMap<String, i128>,
                                    byte_codes: &Vec<ByteCode>) -> bool {
        let byte_code = &byte_codes.get(self.address_current_instruction);
        match byte_code {
            Some(byte_code_) => {
                match byte_code_.execute(coroutines, self, store) {
                    CoroutineState::Pending => { true }
                    CoroutineState::Fulfilled => { false }
                }
            }
            None => {
                false
            }
        }
    }

    pub(crate) fn increment_address_current_instruction(&mut self, byte_code: &ByteCode,
                                             partial_calculation: bool) {
        if partial_calculation {
            return;
        }
        match byte_code {
            ByteCode::Jump { address_instruction } => {
                self.address_current_instruction = address_instruction.clone();
            }
            _ => {
                self.address_current_instruction += 1;
            }
        };
    }
}


pub(crate) enum CoroutineState {
    Pending,
    Fulfilled,
}