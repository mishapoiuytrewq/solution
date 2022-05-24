use std::collections::HashMap;
use crate::ByteCode;
use crate::coroutine::Coroutine;

pub(crate) struct VirtualMachine {
    byte_codes: Vec<ByteCode>,
    store: HashMap<String, i128>,
    coroutines: Vec<Coroutine>,
}

impl VirtualMachine {
    pub(crate) fn new(byte_codes: Vec<ByteCode>) -> VirtualMachine {
        return VirtualMachine {
            byte_codes,
            store: Default::default(),
            coroutines: Default::default(),
        };
    }

    pub(crate) fn run(&mut self) {
        self.coroutines.insert(0, Coroutine::new(0));
        while self.coroutines.len() > 0 {
            let mut new_coroutines: Vec<Coroutine> = vec![];
            let old_coroutines = self.coroutines.clone()
                .into_iter()
                .map(|mut coroutine|
                    {
                        let works = coroutine.execute_byte_code(&mut new_coroutines,
                                                                &mut self.store,
                                                                &self.byte_codes);
                        (coroutine, works)
                    }
                )
                .filter(|(_, works)| works.clone())
                .map(|(coroutine, _)| coroutine)
                .collect::<Vec<Coroutine>>();
            old_coroutines.into_iter()
                .map(|coroutine| new_coroutines.push(coroutine))
                .last();
            self.coroutines = new_coroutines;
        }
    }
}