enum Register {
    A,B,C,SP,PC,BP,FLAGS,
}

trait Addressable{
    
}

// Simulate a computer
struct Machine {
    registers : [u16; 8],
    memory : [u8 ; 5000],
}

impl Machine{
    pub fn new() -> Self {
        Self {
            registers : [0;8],
            memory : [0;5000],
        }
    }

    // The virtual machine binary is going to call step on the vm until the program is done
    // and then an output is returned
    pub fn step(&mut self) -> Result<() , &'static str>{
        let pc = self.registers[Register::PC];
        self.memory.read
    }
}
