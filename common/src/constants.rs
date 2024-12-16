// Architecture-specific constants
pub const XLEN: usize = 32;  // Register length in bits
const RISCV_REGISTER_COUNT: usize = 32;  // Number of general-purpose registers in the RISC-V architecture
const VIRTUAL_REGISTER_COUNT: usize = 32;  // Number of virtual registers, as described in the Jolt paper
pub const REGISTER_COUNT: usize = RISCV_REGISTER_COUNT + VIRTUAL_REGISTER_COUNT;  // Total register count (including virtual registers)
pub const BYTES_PER_INSTRUCTION: usize = 4;  // Number of bytes in one instruction

// Memory operation constants
/// Number of memory operations per instruction (3 registers and 1 RAM access)
pub const MEMORY_OPS_PER_INSTRUCTION: usize = 4;

// Memory-related constants
pub const RAM_START_ADDRESS: u64 = 0x80000000;  // Starting address of RAM
pub const DEFAULT_MEMORY_SIZE: usize = 10 * 1024 * 1024;  // Default memory size (10 MB)
pub const DEFAULT_STACK_SIZE: usize = 4096;  // Default stack size (4 KB)
pub const DEFAULT_MAX_INPUT_SIZE: usize = 4096;  // Default maximum input size (4 KB)
pub const DEFAULT_MAX_OUTPUT_SIZE: usize = 4096;  // Default maximum output size (4 KB)

// Function to calculate the index of a virtual register
pub const fn virtual_register_index(index: usize) -> usize {
    index + VIRTUAL_REGISTER_COUNT  // Virtual registers follow after the physical registers
}

// Layout of the witness (memory layout):
//     registers || virtual registers || inputs || outputs || panic || termination || padding || RAM
// Layout of the VM memory (memory address space):
//     peripheral devices || inputs || outputs || panic || termination || padding || RAM
// The VM memory can be mapped to witness indices with an offset:
//     (RAM_WITNESS_OFFSET + RAM_START_ADDRESS)
