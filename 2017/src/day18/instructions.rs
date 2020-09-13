use super::State;
use std::fmt::Debug;

pub trait Instruction: Debug {
    fn exec(&self, registers: &mut State) -> Option<isize>;
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
            fn exec(&$self, $state: &mut State) -> Option<isize> $body
        }
    };
}

impl_inst! {Add, self, state, a, {
    state.registers[self.r] += self.a;
    None
}}

impl_inst! {AddR, self, state, a, {
    state.registers[self.r] += state.registers[self.a as usize];
    None
}}

impl_inst! {Jgz, self, state, a, {
    if state.registers[self.r] > 0 {
            state.instruction += self.a - 1;
    }
    None
}}

impl_inst! {JgzR, self, state, a, {
    if state.registers[self.r] > 0 {
            state.instruction += state.registers[self.a as usize] - 1;
    }
    None
}}

impl_inst! {Mul, self, state, a, {
    state.registers[self.r] *= self.a;
    None
}}

impl_inst! {MulR, self, state, a, {
    state.registers[self.r] *= state.registers[self.a as usize];
    None
}}

impl_inst! {Mod, self, state, a, {
    state.registers[self.r] %= self.a;
    None
}}

impl_inst! {ModR, self, state, a, {
    state.registers[self.r] %= state.registers[self.a as usize];
    None
}}

impl_inst! {Rcv, self, state, {
    if state.registers[self.r] > 0 {
        return Some(state.frequency);
    }
    None
}}

impl_inst! {Snd, self, state, {
    state.frequency = state.registers[self.r];
    None
}}

impl_inst! {Set, self, state, a,{
    state.registers[self.r] = self.a;
    None
}}

impl_inst! {SetR, self, state, a,{
    state.registers[self.r] = state.registers[self.a as usize];
    None
}}
