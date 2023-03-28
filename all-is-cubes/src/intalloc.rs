//! “Integer allocation”: algorithms for finding small integers not currently in use
//! for the same purpose.
use num_traits::int::PrimInt;
use std::fmt::Debug;
/// An integer allocator; that is, an algorithm for solving the problem
/// “give me an integer that nobody else is currently using”, as tracked
/// by the allocator.
///
/// While this is parameterized over all primitive integers, it will not
/// allocate negative values.
#[derive(Clone, Debug)]
pub struct IntAllocator<T: PrimInt + Debug> {
    /// All larger integers are free. If `None`, then zero has not been allocated.
    last_allocated: Option<T>,
    /// List of integers ≤ than [`Self::last_allocated`] that are free.
    free_list: Vec<T>,
}
impl<T: PrimInt + Debug> IntAllocator<T> {
    pub fn new() -> Self {
        loop {}
    }
    /// Returns an integer not currently allocated, or `None` if all nonnegative
    /// integers of type `T` are already allocated.
    pub fn allocate(&mut self) -> Option<T> {
        loop {}
    }
    /// Makes a previously allocated integer available for use.
    ///
    /// Caution: passing an integer not currently allocated will corrupt the state and
    /// lead to overlapping allocations.
    pub fn free(&mut self, value: T) {
        loop {}
    }
}
impl<T: PrimInt + Debug> Default for IntAllocator<T> {
    fn default() -> Self {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_allocator_reuses_last() {
        loop {}
    }
    #[test]
    fn int_allocator_reuses_nonlast() {
        loop {}
    }
    #[test]
    fn int_allocator_numeric_limit() {
        loop {}
    }
}
