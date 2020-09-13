use super::State;
use std::fmt::Debug;

pub enum RunResult {
    Normal,
    Jump(isize),
    Send(isize),
    Suspend,
}

pub trait Instruction: Debug {
    fn exec(&self, registers: &mut State) -> RunResult;
}

macro_rules! impl_inst {
    ($type:ident, $self:ident, $state:ident, $($member:ident),* $(,)* $body:block) => {
        #[derive(Debug)]
        pub struct $type {
            pub r: usize,
            $(
                pub $member: isize,
            )*
        }

        impl Instruction for $type {
            fn exec(&$self, $state: &mut State) -> RunResult $body
        }
    };
}

impl_inst! {Add, self, state, a, {
    state.registers[self.r] += self.a;
    RunResult::Normal
}}

impl_inst! {AddR, self, state, a, {
    state.registers[self.r] += state.registers[self.a as usize];
    RunResult::Normal
}}

impl_inst! {Jgz, self, state, a, {
    if state.registers[self.r] > 0 {
        return RunResult::Jump(self.a);
    }
    RunResult::Normal
}}

impl_inst! {JgzR, self, state, a, {
    if state.registers[self.r] > 0 {
        return RunResult::Jump(state.registers[self.a as usize]);
    }
    RunResult::Normal
}}

impl_inst! {Mul, self, state, a, {
    state.registers[self.r] *= self.a;
    RunResult::Normal
}}

impl_inst! {MulR, self, state, a, {
    state.registers[self.r] *= state.registers[self.a as usize];
    RunResult::Normal
}}

impl_inst! {Mod, self, state, a, {
    state.registers[self.r] %= self.a;
    RunResult::Normal
}}

impl_inst! {ModR, self, state, a, {
    state.registers[self.r] %= state.registers[self.a as usize];
    RunResult::Normal
}}

impl_inst! {Rcv, self, state, {
    match state.input.pop_front() {
        Some(value) => state.registers[self.r] = value,
        None => return RunResult::Suspend,
    }
    RunResult::Normal
}}

impl_inst! {Snd, self, state, {
    RunResult::Send(state.registers[self.r])
}}

impl_inst! {Set, self, state, a,{
    state.registers[self.r] = self.a;
    RunResult::Normal
}}

impl_inst! {SetR, self, state, a,{
    state.registers[self.r] = state.registers[self.a as usize];
    RunResult::Normal
}}
