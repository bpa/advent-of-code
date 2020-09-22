use super::Program;
use std::fmt::Debug;

pub enum RunResult {
    Normal,
    Jump(isize),
    Send(isize),
    Suspend,
}

#[derive(Debug)]
pub enum Field {
    Register(isize),
    Value(isize),
}

pub trait Instruction: Debug {
    fn exec(&self, registers: &mut Program) -> RunResult;
}

macro_rules! impl_inst {
    ($type:ident, $self:ident, $program:ident, $($member:ident),* $(,)* $body:block) => {
        #[derive(Debug)]
        pub struct $type {
            pub r: Field,
            $(
                pub $member: Field,
            )*
        }

        impl Instruction for $type {
            fn exec(&$self, $program: &mut Program) -> RunResult $body
        }
    };
}

macro_rules! value {
    ($field:expr, $program:ident) => {
        match $field {
            Field::Register(r) => $program.registers[r as usize],
            Field::Value(v) => v,
        }
    };
}

macro_rules! register {
    ($self:ident, $program:ident) => {
        $program.registers[match $self.r {
            Field::Register(r) => r,
            Field::Value(v) => v,
        } as usize]
    };
}

impl_inst! {Add, self, program, a, {
    register![self, program] += value!(self.a, program);
    RunResult::Normal
}}

impl_inst! {Jgz, self, program, a, {
    if value!(self.r, program) > 0 {
        return RunResult::Jump(value!(self.a, program));
    }
    RunResult::Normal
}}

impl_inst! {Mul, self, program, a, {
    register![self, program] *= value![self.a, program];
    RunResult::Normal
}}

impl_inst! {Mod, self, program, a, {
    register![self, program] %= value![self.a, program];
    RunResult::Normal
}}

impl_inst! {Rcv, self, program, {
    match program.input.pop_front() {
        Some(value) => register![self, program] = value,
        None => return RunResult::Suspend,
    }
    RunResult::Normal
}}

impl_inst! {Snd, self, program, {
    RunResult::Send(value![self.r, program])
}}

impl_inst! {Set, self, program, a,{
    register![self, program] = value![self.a, program];
    RunResult::Normal
}}
