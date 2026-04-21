struct CPU {
    PC: u16,    // Program Counter
    AC: u8,     // Accumulator
    X: u8,      // X register
    Y: u8,      // Y register
    SR: u8,     // Status register
    SP: u8,     // Stack pointer
}

const SR_N: u8 = 0b1000_0000;   // Negative
const SR_V: u8 = 0b0100_0000;   // Overflow
const SR_B: u8 = 0b0001_0000;   // Break
const SR_D: u8 = 0b0000_1000;   // Decimal (use for BCD arithmetic)
const SR_I: u8 = 0b0000_0100;   // Interrupt (IRQ disable)
const SR_Z: u8 = 0b0000_0010;   // Zero
const SR_C: u8 = 0b0000_0000;   // Carry

impl CPU {
    func 
}