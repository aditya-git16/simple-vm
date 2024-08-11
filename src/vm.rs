enum Register {
    A,B,C,SP,PC,BP,FLAGS,
}

trait Addressable{
    // 8 bit read and write
    fn read(&self , addr : u16) -> Option<u8>;
    fn write(&mut self , addr: u16, value: u8) -> bool;
    // 16 bit read and write , implemented as default methods
    fn read2(&self , addr : u16) -> Option<u16>{
        // if let is being used for pattern matching
        if let Some(x0) = self.read(addr){
            if let Some(x1) = self.read(addr+1) {
            // | is the bitwise OR operator. << is for left bit shift
            return (x0 as u16) | (x1 as u16 << 8);
            }
        };
        None
    }
    fn write2(&mut self , addr: u16, value: u16) -> bool{
        // value & 0xff performs a bitwise AND operation between value and 0xff (which is 255 in decimal).
        // This operation extracts the lower 8 bits (the least significant byte) of the 16-bit value. The result is assigned to lower
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;
        self.write(addr, lower) && self.write(addr, upper)
    }

    fn copy (&mut self , from :u16 , to:u16 , n: size) -> bool{
        for i in 0..n {
            if let Some(x) = self(from+i){
                if !self.write(to+i, x){
                    return false;
                }
            } else {
                return false;
            }
        }
    }
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
