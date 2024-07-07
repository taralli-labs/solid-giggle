use std::eprintln;

use winter_air::EvaluationFrame;

use super::{enforce_constraints, NUM_CONSTRAINTS, ONE, ZERO};
use crate::{
    trace::{chiplets::ARITH_TRACE_RANGE, TRACE_WIDTH},
    Felt,
};

#[test]
fn test_one_is_one() {
    let mut current = vec![ZERO; TRACE_WIDTH];
    let mut next = vec![ZERO; TRACE_WIDTH];

    // input
    let current_arith = [ONE];
    // output
    let next_arith = [ONE];

    current[ARITH_TRACE_RANGE].copy_from_slice(&current_arith);
    next[ARITH_TRACE_RANGE].copy_from_slice(&next_arith);

    let frame = EvaluationFrame::<Felt>::from_rows(current, next);

    let result = get_constraint_evaluation(frame);
    assert_eq!(result, [ZERO; NUM_CONSTRAINTS]);
}

#[test]
fn test_two_is_not_one() {
    let mut current = vec![ZERO; TRACE_WIDTH];
    let mut next = vec![ZERO; TRACE_WIDTH];

    // input
    let current_arith = [ONE];
    // output
    let next_arith = [ONE + ONE];

    current[ARITH_TRACE_RANGE].copy_from_slice(&current_arith);
    next[ARITH_TRACE_RANGE].copy_from_slice(&next_arith);

    let frame = EvaluationFrame::<Felt>::from_rows(current, next);

    let result = get_constraint_evaluation(frame);
    assert_ne!(result, [ZERO; NUM_CONSTRAINTS]);
}

fn get_constraint_evaluation(frame: EvaluationFrame<Felt>) -> [Felt; NUM_CONSTRAINTS] {
    let mut result = [ZERO; NUM_CONSTRAINTS];

    enforce_constraints(&frame, &mut result, ONE);

    result
}
