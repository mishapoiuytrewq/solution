mod virtual_machine;
mod byte_code;
mod coroutine;

use byte_code::*;
use virtual_machine::*;

fn main() {
    let mut vm = VirtualMachine::new(vec![
        ByteCode::LoadVal { value: 1 },                     // 0
        ByteCode::WriteVar { name: "x".to_string() },       // 1
        ByteCode::LoadVal { value: 2 },                     // 2
        ByteCode::WriteVar { name: "y".to_string() },
        ByteCode::ReadVar { name: "x".to_string() },
        ByteCode::LoadVal { value: 1 },
        ByteCode::Add,
        ByteCode::ReadVar { name: "y".to_string() },
        ByteCode::Multiply,
        ByteCode::PopLog,
        ByteCode::LoadVal { value: 23 },                    // 10
        ByteCode::LoadVal { value: 27 },
        ByteCode::Spawn,
        ByteCode::LoadVal { value: 10 },
        ByteCode::WriteVar { name: "i".to_string() },
        ByteCode::ReadVar { name: "i".to_string() },
        ByteCode::LoadVal { value: -1 },
        ByteCode::Add,
        ByteCode::Log,
        ByteCode::WriteVar { name: "i".to_string() },
        ByteCode::ReadVar { name: "i".to_string() },        // 20
        ByteCode::JumpIfHeadStackTrue { address_instruction: 15 },
        ByteCode::Return,
        //
        ByteCode::SendChannel {
            name: "chan".to_string(),
            value: 10,
        },                                                  // 23
        ByteCode::LoadVal { value: 123 },
        ByteCode::PopLog,
        ByteCode::Return,
        //
        ByteCode::RecvChannel {
            name: "chan".to_string()
        },                                                  // 27
        ByteCode::PopLog,
        ByteCode::LoadVal { value: 321 },
        ByteCode::ReturnValue,                              // 30
    ]);
    vm.run();
}
