use std::eprintln;

use crate::{
    trace::chiplets::ARITH_A_COL_IDX,
    utils::{are_equal, is_binary},
    ONE, ZERO,
};

use super::{EvaluationFrame, Felt, FieldElement};
use crate::trace::chiplets::arith::{OP_CYCLE_LEN, TRACE_WIDTH};
use alloc::vec::Vec;
use winter_air::TransitionConstraintDegree;

pub const NUM_CONSTRAINTS: usize = 2;

pub const ZEROES: [Felt; OP_CYCLE_LEN] = [ZERO; OP_CYCLE_LEN];

pub fn get_periodic_column_values() -> Vec<Vec<Felt>> {
    vec![]
}

pub fn get_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    let degrees: [TransitionConstraintDegree; NUM_CONSTRAINTS] =
        [TransitionConstraintDegree::new(4), TransitionConstraintDegree::new(3)];

    degrees.into()
}

/// Returns the number of transition constraints for the chiplet.
// pub fn get_transition_constraint_count() -> usize {}

pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    processor_flag: E,
) {
    let mut index = 0;

    eprintln!("{} {}", frame.selector(), frame.selector_next());

    result[index] = processor_flag * is_binary(frame.selector());
    index += 1;

    result[index] = processor_flag * are_equal(frame.selector(), frame.selector_next());
    index += 1;
}

pub trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    /// Gets the current value of the specified selector column.
    fn selector(&self) -> E;

    /// Gets the next value of the specified selector column.
    fn selector_next(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    #[inline(always)]
    fn selector(&self) -> E {
        self.current()[ARITH_A_COL_IDX]
    }

    #[inline(always)]
    fn selector_next(&self) -> E {
        self.next()[ARITH_A_COL_IDX]
    }
}

#[cfg(test)]
pub mod tests;
