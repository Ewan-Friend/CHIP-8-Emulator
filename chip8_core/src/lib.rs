const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;

const MEM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDRESS: u16 = 0x200;

pub struct Chip8 {
    pc: u16,
    memory: [u8; MEM_SIZE],
    frame_buffer: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    keys: [u16; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            pc: START_ADDRESS,
            memory: [0; MEM_SIZE],
            frame_buffer: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            stack: [0; STACK_SIZE],
            keys: [0; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }
}
