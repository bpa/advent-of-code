use std::boxed::Box;
use std::fmt::Debug;

pub trait Condition: Debug {
    fn eval(&self, registers: &mut Vec<isize>) -> bool;
}

macro_rules! impl_condition {
    ($op:ident, $infix:tt) => {
        #[derive(Debug)]
        pub struct $op {
            register: usize,
            value: isize,
        }

        impl $op {
            pub fn new(register: usize, value: isize) -> Self {
                $op { register, value }
            }
        }

        impl Condition for $op {
            fn eval(&self, registers: &mut Vec<isize>) -> bool {
                registers[self.register] $infix self.value
            }
        }
    };
}

impl_condition!(LT, <);
impl_condition!(GT, >);
impl_condition!(LTE, <=);
impl_condition!(GTE, >=);
impl_condition!(EQ, ==);
impl_condition!(NE, !=);

pub trait Update: Debug {
    fn update(&self, registers: &mut Vec<isize>) -> isize;
}

macro_rules! impl_update {
    ($op:ident, $action:tt) => {
        #[derive(Debug)]
        pub struct $op {
            register: usize,
            amount: isize,
            condition: Box<dyn Condition>,
        }

        impl $op {
            pub fn new(register: usize, amount: isize, condition: Box<dyn Condition>) -> Self {
                $op { register, amount, condition }
            }
        }

        impl Update for $op {
            fn update(&self, registers: &mut Vec<isize>) -> isize {
                if self.condition.eval(registers) {
                    registers[self.register] $action self.amount;
                }
                registers[self.register]
            }
        }
    };
}

impl_update! {Inc, +=}
impl_update! {Dec, -=}
