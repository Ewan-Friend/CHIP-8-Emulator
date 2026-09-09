const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;

const MEM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDRESS: u16 = 0x200;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80  //F
];

pub struct Chip8 {
    pc: u16,
    memory: [u8; MEM_SIZE],
    frame_buffer: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [u16; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8_default = Self {
            pc: START_ADDRESS,
            memory: [0; MEM_SIZE],
            frame_buffer: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [0; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        chip8_default.memory[0..FONTSET_SIZE].copy_from_slice(&FONTSET);

        chip8_default
    }

    pub fn wipe(&mut self) {
        self.pc = START_ADDRESS;
        self.memory = [0; MEM_SIZE];
        self.frame_buffer = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [0; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.memory[0..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn push(&mut self , val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn tick(&mut self) {
        let op = self.fetch();

        self.run(op);
    }

    fn split_digits(&mut self, op: u16) -> (u8, u8, u8, u8){
        let d1 = ((op & 0xF000) >> 12) as u8;
        let d2 = ((op & 0x0F00) >> 8) as u8;
        let d3 = ((op & 0x00F0) >> 4) as u8;
        let d4 = (op & 0x000F) as u8;

        (d1, d2, d3, d4)
    }

    fn run(&mut self, op: u16){
        //TODO
        //Match patterns of opcodes to specific events
        //Doing this one myself!

        let digits = self.split_digits(op);

        match digits {
            // 0000
            // NOP
            // Does nothing
            (0, 0, 0, 0) => {
                return
            },
            // 00E0
            // CLS
            // Clear the display
            (0, 0, 0xE, 0) => {
                self.frame_buffer = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            },
            // 00EE
            // RET
            // Return from subroutine
            (0, 0, 0xE, 0xE) => {
                self.pc = self.pop();
            },
            // 1nnn
            // JP addr
            // Jumps to location nnn
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            },
            // 2nnn 
            // CALL addr 
            // Call subroutine at nnn 
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn; 
            },
            // 3xkk
            // SE Vx, byte 
            // Skip the next instruction if Vx = kk
            (3, _, _, _) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let kk = (op & 0xFF) as u8;

                if self.v_reg[x] == kk {
                    self.pc += 2;
                }
            },
            // 4xkk 
            // SNE Vx, byte
            // Skip the next instruction if Vx != kk
            (4, _, _, _) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let kk = (op & 0xFF) as u8;

                if self.v_reg[x] != kk {
                    self.pc += 2;
                }
            },
            // 5xy0
            // SE Vx, Vy
            // Skip the next instruction if Vx == Vy 
            (5, _, _, 0) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },
            // 6xkk 
            // LD Vx, byte
            // Set Vx = kk
            (6, _, _, _) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let kk = (op & 0xFF) as u8;

                self.v_reg[x] = kk;
            },
            // 7xkk 
            // ADD Vx, byte 
            // Set Vx = Vx + kk
            (7, _, _, _) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let kk = (op & 0xFF) as u8;

                let acc = self.v_reg[x];
                self.v_reg[x] = kk + acc;
            },
            // 8xy0 
            // LD Vx, Vy 
            // Stores value of register Vy in Register Vx
            (8, _, _, 0) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                self.v_reg[x] = self.v_reg[y];
            },
            // 8xy1
            // OR Vx, Vy
            // Set Vx = Vx OR Vy
            (8, _, _, 1) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                let result = self.v_reg[x] | self.v_reg[y];
                self.v_reg[x] = result;
            },
            // 8xy2
            // AND Vx, Vy
            // Set Vx = Vx OR Vy
            (8, _, _, 2) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                let result = self.v_reg[x] & self.v_reg[y];
                self.v_reg[x] = result;
            },
            // 8xy3
            // XOR Vx, Vy
            // Set Vx = Vx XOR Vy
            (8, _, _, 3) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                let result = self.v_reg[x] ^ self.v_reg[y];
                self.v_reg[x] = result;
            },
            // 8xy4
            // ADD Vx, Vy
            // Set Vx = Vx + Vy, set VF = carry
            (8, _, _, 4) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                let (result, carry) = self.v_reg[x]
                    .overflowing_add(self.v_reg[y]);

                self.v_reg[x] = result;
                self.v_reg[0xF] = if carry {1} else {0};
            },
            // 8xy5
            // SUB Vx, Vy
            // Set Vx = Vx - Vy, set VF = NOT borrow
            (8, _, _, 5) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                let (result, borrow) = self.v_reg[x]
                    .overflowing_sub(self.v_reg[y]);

                self.v_reg[x] = result;
                self.v_reg[0xF] = if borrow {0} else {1};
            },
            // 8xy6
            // SHR Vx {, Vy}
            // Set Vx = Vx SHR 1
            (8, _, _, 6) => {
                let x = ((op & 0xF00) >> 8) as usize;

                let remainder = self.v_reg[x] & 0b1;

                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = remainder;
            },
            // 8xy7
            // SUBN Vx, Vy
            // Set Vx = Vy - Vx, set VF = NOT borrow
            (8, _, _, 7) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                let (result, borrow) = self.v_reg[y]
                    .overflowing_sub(self.v_reg[x]);

                self.v_reg[x] = result;
                self.v_reg[0xF] = if borrow {0} else {1};
            },
            // 8xyE
            // SHL Vx {, Vy}
            // Set Vx = Vx SHL 1
            (8, _, _, 0xE) => {
                let x = ((op & 0xF00) >> 8) as usize;

                let overflow = (self.v_reg[x] >> 7) & 0b1;

                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = overflow;
            },
            // 9xy0
            // SNE Vx, Vy
            // Skip next insturction if Vx != Vy
            (9, _, _, 0) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x0F0) >> 4) as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }
            // Annn
            // LD I, addr
            // Set I = nnn 
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;

                self.i_reg = nnn;
            }
            // Bnnn
            // JP V0, addr
            // Jump to location nnn + v0
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;

                self.pc = nnn + (self.v_reg[0] as u16);
            }
            // Cxkk
            // RND Vx, byte
            // Set Vx = random byte AND kk
            // ALWAYS KEEP LAST
            (_, _, _, _) => unimplemented!("This opcode has not yet been implemented: {}", op),
        };
    }

    fn fetch(&mut self) -> u16 {
        let b1 = self.memory[self.pc as usize] as u16;
        let b2 = self.memory[(self.pc + 1) as usize] as u16;
        let op = (b1 << 8) | b2;
        self.pc += 2;
        op
    }

    pub fn timer_tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // TODO: Make beep
            }

            self.st -= 1;
        }
    }
}
