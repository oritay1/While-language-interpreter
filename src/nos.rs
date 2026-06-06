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
        /* 
            please insert your implementation here
        */
        Stm::Comp(s1,s2) => {
            let s_prime = nos((*s1, state));
            nos((*s2,s_prime))
        }

        // If: [if_tt] and [if_ff]
        Stm::If(b, s1, s2) => {
            if solve_b(&b, &state) {
                nos((*s1, state))
            } else {
                nos((*s2, state))
            }
        }

        // While: [while_tt] and [while_ff]
        /* 
            We clone 'b' and 's' because we are going to use them over and over in each
            loop iteration.
        */
        Stm::While(b,s) => {
            // [while_tt]
            if solve_b(&b, &state) {
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
    }
}