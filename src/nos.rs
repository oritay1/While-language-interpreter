use crate::ast::*;
use crate::semantics::*;

// The main Natural Operational Semantics function:
// nos: (Stm, State) -> State
pub fn nos(c: (Stm, State)) -> State {
    let (stm, state) = c; // Destructuring the tuple into 2 variables

    match stm {
        // Assignment: [ass]
        Stm::Ass(x, e) => update(&x, &e, &state),

        // Skip: [skip]
        Stm::Skip => state,

        // Composition: [comp]
        Stm::Comp(s1,s2) => {
            let s_prime = nos((*s1, state));
            nos((*s2,s_prime))
        }

        // If: [if_tt] and [if_ff]
        Stm::If(b, s1, s2) => {
            if solve_b(&b, &state) == "tt" {
                nos((*s1, state))
            } else {
                nos((*s2, state))
            }
        }

        // While: [while_tt] and [while_ff]
        Stm::While(b,s) => {
            // [while_tt]
            if solve_b(&b, &state) == "tt" {
                // Do s
                let s_prime = nos((*s.clone(), state));
                // Continue loop
                nos((Stm::While(b.clone(), s.clone()), s_prime))
            }
            // [while_ff]
            else {
                state
            }
        }

        // DoWhile [Dowhile_tt] and [Dowhile_ff]
        Stm::DoWhile(s,b) => {
            // Do s
            let s_prime = nos((*s.clone(), state));
            if solve_b(&b, &s_prime ) == "tt" {
                // Continue loop
                nos((Stm::DoWhile(s.clone(), b.clone()), s_prime))
            }
            else {
                s_prime
            }
        }
    }
}