use argh::FromArgs;

use std::fs;
use std::io;

#[derive(Debug, Clone)]
struct Machine{
    memory: Vec<i32>,
    instruction_ptr: usize,
    rel_base_ptr: usize,
}

impl Machine{
    fn new(data: Vec<i32>) -> Machine{
        Machine {
            memory: data,
            instruction_ptr: 0,
            rel_base_ptr: 0
        }
    }

    //Descriptions copied from https://esolangs.org/wiki/Intcode#Opcodes
    //Adds the first two arguments and stores the result in the third argument.
    fn add(&mut self, param_modes: Vec<i32>) -> i32{
        let sum: i32 = (0..2).map(|i| {
            let ptr_to_value = self.ptr_to_value(i+1, &param_modes);

            self.memory[ptr_to_value as usize]
        }).sum();

        let ptr_to_result = self.ptr_to_value(3, &param_modes);
        self.memory[ptr_to_result] = sum;

        3
    }

    //Like 1, but for multiplication.
    fn multiply(&mut self, param_modes: Vec<i32>) -> i32{
        let res: i32 = (0..2).map(|i| {
            let ptr_to_value = self.ptr_to_value(i+1, &param_modes);

            self.memory[ptr_to_value as usize]
        }).product();

        let ptr_to_result = self.ptr_to_value(3, &param_modes);
        self.memory[ptr_to_result] = res;

        3
    }

    //Inputs a single integer and stores it in the first argument.
    fn input(&mut self, param_modes: Vec<i32>) -> i32{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        
        let value: i32 = line.trim().parse().expect("Input not an integer");

        let ptr_to_result = self.ptr_to_value(1, &param_modes);
        self.memory[ptr_to_result] = value;

        1
    }

    //Outputs the first argument.
    fn output(&mut self, param_modes: Vec<i32>) -> i32{
        let ptr_to_result = self.ptr_to_value(1, &param_modes);
        let out_val = self.memory[ptr_to_result];

        print!("{}", out_val as u8 as char);

        1
    }

    //If the first argument is non-zero, sets the instruction pointer to second argument.
    fn set_ip_if_arg_not_zero(&mut self, param_modes: Vec<i32>) -> i32{
        let ptr_arg1 = self.ptr_to_value(1, &param_modes);

        if self.memory[ptr_arg1] != 0 {
            let ptr_arg2 = self.ptr_to_value(2, &param_modes);

            self.instruction_ptr = self.memory[ptr_arg2] as usize;

            return -1
        }

        2
    }

    //Like 5, but jumps if the first argument is zero.
    fn jump_arg_zero(&mut self, param_modes: Vec<i32>) -> i32{
        let ptr_arg1 = self.ptr_to_value(1, &param_modes);

        if self.memory[ptr_arg1] == 0 {
            let ptr_arg2 = self.ptr_to_value(2, &param_modes);

            self.instruction_ptr = self.memory[ptr_arg2] as usize;

            return -1
        }

        2
    }

    //If the first argument is less than the second argument, writes 1 to the third argument. Otherwise, writes 0.
    fn lt_comparison(&mut self, param_modes: Vec<i32>) -> i32{
        let ptr_arg1 = self.ptr_to_value(1, &param_modes);
        let ptr_arg2 = self.ptr_to_value(2, &param_modes);
        let ptr_arg3 = self.ptr_to_value(3, &param_modes);

        if self.memory[ptr_arg1] < self.memory[ptr_arg2] {
            self.memory[ptr_arg3] = 1;
        }else{
            self.memory[ptr_arg3] = 0;
        }

        3
    }

    //Like 7, but check equality instead.
    fn eq_comparison(&mut self, param_modes: Vec<i32>) -> i32{
        let ptr_arg1 = self.ptr_to_value(1, &param_modes);
        let ptr_arg2 = self.ptr_to_value(2, &param_modes);
        let ptr_arg3 = self.ptr_to_value(3, &param_modes);

        if self.memory[ptr_arg1] == self.memory[ptr_arg2] {
            self.memory[ptr_arg3] = 1;
        }else{
            self.memory[ptr_arg3] = 0;
        }

        3
    }

    //Adds the first argument to the relative base register.
    fn add_to_rbr(&mut self, param_modes: Vec<i32>) -> i32{
        let ptr_arg1 = self.ptr_to_value(1, &param_modes);
        self.rel_base_ptr = (self.rel_base_ptr as i32 + self.memory[ptr_arg1]) as usize;
        1
    }
    

    fn parameter_modes(&self, data: i32) -> Vec<i32>{
        let mut num = data/100;
        let mut params = vec![0, 0, 0];

        for i in 0..3{
            params[i] = num % 10;
            num = num/10;
        }

        params
    }

    /*
    https://esolangs.org/wiki/Intcode#Parameter_Modes:
    Parameter modes impact how arguments are read or written. Three parameter modes are defined.

        Mode 0, position mode: the parameter is the address of a cell to be read or written.
        Mode 1, immediate mode: the parameter is the value read. (This mode is never used for writing.)
        Mode 2, relative mode: the parameter is added to the relative base register to obtain the address of the cell to be read or written.
    */
    fn ptr_to_value(&self, arg_num: usize, param_modes: &Vec<i32>) -> usize{
        let argument_index = self.instruction_ptr + arg_num;
        let argument_value = self.memory[argument_index];

        match param_modes[arg_num-1]{
            0 => argument_value as usize,
            1 => self.instruction_ptr + arg_num,
            2 => self.rel_base_ptr + argument_value as usize,
            _ => panic!("Param mode not recognized: address {}", self.instruction_ptr)
        }
    }

    //Executes the program saved in self.memory
    fn execute(&mut self){
        loop{
            let data = self.memory[self.instruction_ptr];

            let opcode = data % 100;
            let param_modes = self.parameter_modes(data);

            //Match every opcode with the corresponding function
            let skip = match opcode {
                1 => self.add(param_modes),
                2 => self.multiply(param_modes),
                3 => self.input(param_modes),
                4 => self.output(param_modes),
                5 => self.set_ip_if_arg_not_zero(param_modes),
                6 => self.jump_arg_zero(param_modes),
                7 => self.lt_comparison(param_modes),
                8 => self.eq_comparison(param_modes),
                9 => self.add_to_rbr(param_modes),
                99 => break,
                _ => panic!("Opcode not recognized: op{}, addr {}", self.memory[self.instruction_ptr], self.instruction_ptr)
            };

            self.instruction_ptr = (self.instruction_ptr as i32 + skip + 1) as usize;
            //println!();
        }
    }
}

fn main() {
    let options: ConfigOptions = argh::from_env();
    let data: String = fs::read_to_string(options.filename)
        .expect("No file selected")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let data: Vec<_> = data.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut interpreter = Machine::new(data);

    interpreter.execute();
}

#[derive(FromArgs)]
///Intcode interpreter
struct ConfigOptions{
    #[argh(option, short = 'f')]
    ///name of intcode file.
    filename: String
}