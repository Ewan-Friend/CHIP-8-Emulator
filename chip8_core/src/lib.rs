use rand;
use std::io;

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
    keys: [bool; NUM_KEYS],
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
            keys: [false; NUM_KEYS],
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
        self.keys = [false; NUM_KEYS];
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
            },
            // Annn
            // LD I, addr
            // Set I = nnn 
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;

                self.i_reg = nnn;
            },
            // Bnnn
            // JP V0, addr
            // Jump to location nnn + v0
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;

                self.pc = nnn + (self.v_reg[0] as u16);
            },
            // Cxkk
            // RND Vx, byte
            // Set Vx = random byte AND kk
            (0xC, _, _, _) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let kk = (op & 0xFF) as u8;
                let byte = rand::random::<u8>();

                self.v_reg[x] = byte & kk;
            },
            // Dxyn
            // DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I 
            // at (Vx, Vy), set VF = collision
            (0xD, _, _, _) => {
                let x = ((op & 0xF00) >> 8) as u16;
                let y = ((op & 0x0F0) >> 4) as u16;
                let n = op & 0xF;

                let mut toggled: bool = false;

                // Iterate through rows of the screen
                for yline in 0..n {
                    let addr = self.i_reg + yline as u16;
                    let pixels = self.memory[addr as usize];
                    
                    for xline in 0..8 {
                        if (pixels & (0b00000001 >> xline)) != 0 {
                            // Wrap around
                            let x = (x + xline) as usize % SCREEN_WIDTH;
                            let y = (y + yline) as usize % SCREEN_HEIGHT;

                            // 1D -> 2D
                            let idx = y * SCREEN_WIDTH + x;

                            toggled |= self.frame_buffer[idx];
                            self.frame_buffer[idx] ^= true;
                        }
                    } 
                }

                self.v_reg[0xF] = if toggled {1} else {0}
            },
            // Ex9E 
            // SKP Vx
            // Skip next instruction if key with the value of Vx is pressed
            (0xE, _, 9, 0xE) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let rune = self.v_reg[x];

                if self.keys[rune as usize] {
                    self.pc += 2;
                }
            },
            // ExA1 
            // SKNP Vx
            // Skip next instruction if the key with value of Vx is not pressed
            (0xE, _, 0xA, 1) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let rune = self.v_reg[x];

                if !self.keys[rune as usize] {
                    self.pc += 2;
                }
            },
            // Fx07 
            // LD Vx, DT
            // Set Vx = delay timer value
            (0xF, _, 0, 7) => {
                let x = ((op & 0xF00) >> 8) as usize;
                self.v_reg[x] = self.dt;
            },
            // Fx0A
            // LD Vx, K
            // Wait for a key press, store the value of the key in Vx
            (0xF, _, 0, 0xA) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let mut toggled = false;
                
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8; 
                        toggled = true;
                        break;
                    }
                }

                if !toggled {
                    self.pc -= 2;
                }
            }
            // Fx15
            // LD DT, Vx
            // DT is set equal to the value of Vx
            (0xF, _, 1, 5) => {
                let x = ((op & 0xF00) >> 8) as usize;
                self.dt = self.v_reg[x];
            }
            // Fx18
            // LD ST, Vx
            // Set I = I + Vx
            (0xF, _, 1, 8) => {
                let x = ((op & 0xF00) >> 8) as usize;
                self.st = self.v_reg[x];
            }
            // Fx1E
            // ADD I, Vx
            // Set I = I + Vx
            (0xF, _, 1, 0xE) => {
                let x = ((op & 0xF00) >> 8) as usize;
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[x] as u16);
            },
            // Fx29
            // LD F, Vx
            // Set I = location of sprite for digit Vx
            (0xF, _, 2, 9) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let font_addr = (self.v_reg[x] as u16) * 5;
                self.i_reg = font_addr;
            }
            // Fx33
            // LD B, Vx
            // Store BCD representation of Vx in memory locations I, I + 1, and I + 2
            (0xF, _, 3, 3) => {
                let x = ((op & 0xF00) >> 8) as usize;
                let n = self.v_reg[x];

                self.memory[self.i_reg as usize] = n / 100;
                self.memory[(self.i_reg + 1) as usize] = (n / 10) % 10;
                self.memory[(self.i_reg + 2) as usize] = n % 10;
            }
            // Fx55
            // LD [I], Vx
            // Store registers V0 through Vx in memory starting at location I
            (0xF, _, 5, 5) => {
                let x = ((op & 0xF00) >> 8) as usize;

                for i in 0..=x {
                    let val = self.v_reg[i];
                    self.memory[(self.i_reg as usize) + i] = val;
                }
            }
            // Fx65
            // LD Vx, [I]
            // Read registers V0 through Vx from memory starting at location I
            (0xF, _, 6, 5) => {
                let x = ((op & 0xF00) >> 8) as usize;

                for i in 0..x {
                    let val = self.memory[(self.i_reg as usize) + i];
                    self.v_reg[i] = val;
                }
            }
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

    pub fn get_display(&self) -> &[bool] {
        &self.frame_buffer
    }

    pub fn keypress(&mut self, i: usize, pressed:bool) {
        self.keys[i] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let s= START_ADDRESS as usize;
        let e= (START_ADDRESS as usize) + data.len();
        self.memory[s..e].copy_from_slice(data);
    }
}
