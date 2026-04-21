struct CPU {
    PC: u16,    // Program Counter
    AC: u8,     // Accumulator
    X: u8,      // X register
    Y: u8,      // Y register
    SR: u8,     // Status register
    SP: u8,     // Stack pointer

    Memory: &Memory,
}

const SR_N: u8 = 0b1000_0000;   // Negative
const SR_V: u8 = 0b0100_0000;   // Overflow
const SR_B: u8 = 0b0001_0000;   // Break
const SR_D: u8 = 0b0000_1000;   // Decimal (use for BCD arithmetic)
const SR_I: u8 = 0b0000_0100;   // Interrupt (IRQ disable)
const SR_Z: u8 = 0b0000_0010;   // Zero
const SR_C: u8 = 0b0000_0000;   // Carry

#[derive(Default)]
struct Memory {
    data: [u8; 0x10000],
}

impl Memory {
    func get(&self, address: u16) -> u8 {
        self.data[address]
    }

    func set(&mut self, address: u16, data: u8) {
        self.data[address] = data;
    }
}

impl CPU {
    func ADC(&self, inputs: &[u8]) {

    }

    func AM_Acc(&self, inputs: &[u8]) -> u8 {

    }
}