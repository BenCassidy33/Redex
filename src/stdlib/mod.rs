use crate::ast::{
    assignments::{Assignments, insert_assignment},
    expr::{Statement, Unzip},
};

use std::collections::HashMap;

pub mod numerals;

macro_rules! generate_stdlib_assignments {
    ($({$ident:expr => $($value:tt)+}),* $(,)?) => {
        pub fn stdlib_assignments() -> Assignments {
            let mut map = HashMap::new();
            $(
                let assignment = Statement::parse(&format!("{} := {}", stringify!($ident), stringify!($($value)*)))
                    .unwrap().unzip().assignments[0].clone();
                if insert_assignment(&mut map, assignment).is_some() {
                    panic!("Standard library variables cannot have duplicates!")
                };
            )*

            map
        }
    }
}

//TODO: Check accuracy of these
// https://en.wikipedia.org/wiki/Lambda_calculus#Logic_and_predicates
generate_stdlib_assignments! {
    { 0 => Lf.Lx.x },
    { I => Lx.x },
    { S => Lx.Ly.Lz.xyz },
    { K => Lx.Ly.x },
    { B => Lx.Ly.Lz.xyz },
    { C => Lx.Ly.Lz.xzy },
    { W => Lx.Ly.xyy },
    { U => (Lx.x)x },
    { OMEGA => U U },
    { Y => BU(C(B(U))) },

    { TRUE => Lx.Ly.x },
    { FALSE => Lx.Ly.y },

    { AND => Lp.Lq.((pq)p) },
    { OR => Lp.Lq.ppq },
    { NOT => Lp.p(TRUE FALSE) },
    { IFTHENELSE => Lp.La.Lb.(pab) },
    //
    { SUCC => Ln.Lf.Lx.(f((nf)x)) },
    { PLUS => Lm.Ln.(m SUCC n) },
    { SUB => Lm.Ln.(n PRED m) },
    { MULT => Lm.Ln.((m PLUS n) 0) },
    { POW => Lb.Ln.(nb) },

    { PAIR => Lx.Ly.Lf.(f(xy)) },

    { FIRST => Lp.(p(Lx.Ly.x)) },
    { SECOND => Lp.(p(Lx.Ly.y)) },
    { NULL => Lp.p(Lx.Ly.FALSE) },

    { NIL => Lf.TRUE },

    { ISZERO => Ln.(n (Lx.FALSE) TRUE) },
    { LEQ => Lm.Ln.(ISZERO(SUB mn)) },

    { PREDICATE => Ln.(n(Lg.Lk.ISZERO)(g1)k(PLUS((gk)1))(Lv.0)0) }
}
