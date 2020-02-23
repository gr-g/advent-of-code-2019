use std::collections::VecDeque;
use std::convert::TryInto;

#[derive(Clone)]
pub struct IntCode {
    pub memory: Vec<i64>,     // memory
    pub input: VecDeque<i64>, // input buffer
    pub output: Vec<i64>,     // output buffer
    pub display: bool,        // switch display on/off
    ptr: usize,               // instruction pointer
    base: i64,                // relative base
}

impl IntCode {
    pub fn new( program: &[i64] ) -> IntCode {
        IntCode {
            memory: program.to_vec(),
            input: VecDeque::new(),
            output: Vec::new(),
            display: true,
            ptr: 0,
            base: 0,
        }
    }
    
    pub fn reset( &mut self, program: &[i64] ) {
        self.memory.clear();
        self.memory.extend_from_slice(program);
        self.input.clear();
        self.output.clear();
        self.ptr = 0;
        self.base = 0;
    }

    pub fn is_halted( &self ) -> bool {
        self.memory[self.ptr] == 99
    }
    
    pub fn run( &mut self ) {
        loop {
            let (op, op_len, (a1, a2, a3)) = self.decode(self.memory[self.ptr]);
            match op {
                 1 => { self.memory[a3] = self.memory[a1] + self.memory[a2]; self.ptr += op_len; },
                 2 => { self.memory[a3] = self.memory[a1] * self.memory[a2]; self.ptr += op_len; },
                 3 => { if self.input.is_empty() { break; }; self.memory[a1] = self.input.pop_front().unwrap(); self.ptr += op_len; },
                 4 => { self.output.push(self.memory[a1]); self.ptr += op_len; },
                 5 => { if self.memory[a1] != 0 { self.ptr = self.memory[a2].try_into().unwrap(); } else { self.ptr += op_len; } },
                 6 => { if self.memory[a1] == 0 { self.ptr = self.memory[a2].try_into().unwrap(); } else { self.ptr += op_len; } },
                 7 => { if self.memory[a1] < self.memory[a2] { self.memory[a3] = 1; } else { self.memory[a3] = 0; }; self.ptr += op_len; },
                 8 => { if self.memory[a1] == self.memory[a2] { self.memory[a3] = 1; } else { self.memory[a3] = 0; }; self.ptr += op_len; },
                 9 => { self.base += self.memory[a1]; self.ptr += op_len; },
                99 => break,
                 _ => panic!("invalid opcode {}", op),
            }
        }
    }
    
    pub fn run_ascii_command( &mut self, command: &str ) -> (String, Vec<i64>) {
        // load command
        if !command.is_empty() {
            if self.display {
                println!("{}", command);
            }
            for c in command.bytes() {
                self.input.push_back(c as i64);
            }
            self.input.push_back(b'\n' as i64);
        }
        
        // execute
        self.run();
        
        // extract output
        let mut s = String::new();
        let mut rests = vec!();
        
        for c in self.output.drain(..) {
            match c {
                n if n > 0 && n < 128 => { s.push(n as u8 as char); },
                _ => rests.push(c),
            }
        }
        if self.display {
            print!("{}", s);
        }
        (s, rests)
    }
    
    // set the opcode, its length, and the parameter addresses for the current instruction
    fn decode( &mut self, mut instr: i64 ) -> (i64, usize, (usize, usize, usize)) {
        let op = instr % 100;
        instr /= 100;
        
        let op_len = match op {
            1|2|7|8 => 4,
            3|4|9 => 2,
            5|6 => 3,
            99 => 1,
            _ => panic!("invalid opcode {}", op),
        };
        
        let mut addr = [0; 3];
        for p in 0..op_len-1 {
            let ptr = self.ptr+1+p;
            addr[p] = match instr % 10 {
                0 => self.memory[ptr].try_into().unwrap(),
                1 => ptr,
                2 => (self.base + self.memory[ptr]).try_into().unwrap(),
                _ => panic!("invalid instruction {}", instr)
            };
            if addr[p] >= self.memory.len() {
                self.memory.resize(addr[p]+1, 0);
            }
            instr /= 10;
        }
        (op, op_len, (addr[0], addr[1], addr[2]))
    }
}
