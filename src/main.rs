use argh::FromArgs;

use std::fs;

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

    fn execute(&mut self){

    }
}

/*
Parameter Modes as described on https://esolangs.org/wiki/Intcode
Parameter modes impact how arguments are read or written. Three parameter modes are defined.

Mode 0, position mode: the parameter is the address of a cell to be read or written.
Mode 1, immediate mode: the parameter is the value read. (This mode is never used for writing.)
Mode 2, relative mode: the parameter is added to the relative base register to obtain the address of the cell to be read or written.
*/

//Opcodes as described on https://esolangs.org/wiki/Intcode
enum Opcodes{
    Add,             //Adds the first two arguments and stores the result in the third argument.
    Mult,            //Like 1, but for multiplication.
    Input,           //Inputs a single integer and stores it in the first argument.
    Out,             //Outputs the first argument.
    SetIPArgNotZero, //If the first argument is non-zero, sets the instruction pointer to second argument. 
    JumpArgZero,     //Like 5, but jumps if the first argument is zero.
    IsLT,            //If the first argument is less than the second argument, writes 1 to the third argument. Otherwise, writes 0.
    IsEqual,         //Like 7, but check equality instead.
    AddToRBR,        //Adds the first argument to the relative base register.
    Halt,            //Halts the program.
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