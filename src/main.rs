use std::fs::File;
use std::env;
use std::io::Read;
use std::iter::Iterator;

fn load_file(filename: String) -> String{
    let mut f = File::open(filename).unwrap();
    let mut buff = String::new();

    f.read_to_string(&mut buff);
    buff
}

fn get_filename() -> String{
    if env::args().len() < 2{
        panic!("Please provide a filename");
    }

    env::args().nth(1).unwrap()
}

struct Environment{
    registers: Vec<usize>,
    instructions: Vec<char>,
    jumps: Vec<usize>,
    current_register: usize,
    current_instruction: usize
}

impl Environment{
    fn init(code: String) -> Self{
        Environment{
            registers: Vec::with_capacity(30000),
            instructions: code.chars().collect::<Vec<char>>(),
            jumps: Vec::with_capacity(50),
            current_register: 0,
            current_instruction: 0
        }
    }

    /// Primitive instruction "+" adds one to the current register
    fn plus(&mut self){
        self.registers[self.current_register] = self.registers[self.current_register]+1;
        self.next_instruction();
    }

    /// Primitive instruction "-" substracts one from the current register
    fn min(&mut self){
        self.registers[self.current_register] = self.registers[self.current_register]-1;
        self.next_instruction();
    }

    /// Primitive instruction ">" moves the register index to the next register 
    fn next_register(&mut self){
        self.registers.push(0);
        self.current_register += 1;
        self.next_instruction();
    }

    /// Primitive instruction "<" moves the register index to the previous register
    fn previous_register(&mut self){
        self.current_register -= 1;
        self.next_instruction();
    }

    /// Primitive instruction "." prints the value of the current register
    fn print(&mut self){
        println!("{}",self.registers[self.current_register]);
        self.next_instruction();
    }
    
    /// Primitive instruction "[" jumps to the instruction after a matching "]" if the current
    /// register equals to zero, and reads the next instruction if greater than zero
    fn jump_open(&mut self) {
        if self.read_register() > 0{
            self.jumps.push(self.current_instruction);
            self.next_instruction();
        } else {
            self.jump_after_closed();
        }
    }

    /// Primitive instruction "]" jumps back to the matching "["
    fn jump_closed(&mut self) {
        self.current_instruction = self.jumps.pop().unwrap();
        let instr = self.get_current_instruction();
        self.eval(instr);
    }

    /// Every other symbols than "+ - < > , . ] [" get ignored and will read and evaluate the next
    /// instruction
    fn ignore(&mut self){
        self.next_instruction();
    }

    /// helper function that gets the next instruction and evaluates it
    fn next_instruction(&mut self){
        let n = self.current_instruction;

        if n == self.instructions.len()-2{
            return
        }

        let instr = self.get_instruction(n+1);
        self.current_instruction += 1;
        self.eval(instr);
    }

    /// reads the register at the current index
    fn read_register(&self) -> usize{
        self.registers.get(self.current_register).unwrap().to_owned()
    }

    /// gets the current instruction
    fn get_current_instruction(&mut self) -> String{
        let addr = self.current_instruction;
        self.get_instruction(addr)
    }

    /// gets the instruction at index idx
    fn get_instruction(&mut self, idx: usize) -> String{
        self.instructions[idx].to_lowercase().collect()
    }


    /// loops over instruction list until matching "]" is found
    fn jump_after_closed(&mut self){
        while self.get_current_instruction() != "]"{
            self.current_instruction += 1;
        }
        
        self.next_instruction();
    }

    /// main evaluate function, takes in a symbol (instruction) and evaluates it
    fn eval(&mut self, symbol: String){
        match symbol.as_str(){
            ">" => self.next_register(),
            "<" => self.previous_register(),
            "+" => self.plus(),
            "-" => self.min(),
            "." => self.print(),
            "[" => self.jump_open(),
            "]" => self.jump_closed(),
            _ => self.ignore() // ignore everything else
        }
    }

    /// run the interpreter
    fn run(&mut self){
        self.registers.push(0);
        let instr = self.get_instruction(0);
        self.eval(instr);
    }
}

fn main() {
    let filename = get_filename();
    let contents = load_file(filename);
    let mut env = Environment::init(contents);
    env.run();
}
