use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::mem;
use crate::transaction::{Merge, TransactionConflict};
impl<K, V> Merge for BTreeMap<K, V>
where
    K: Clone + Ord + 'static,
    V: Default + Merge,
{
    type MergeCheck = BTreeMap<K, <V as Merge>::MergeCheck>;
    fn check_merge<'a>(
        &'a self,
        mut map2: &'a Self,
    ) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(mut self, mut other: Self, mut check: Self::MergeCheck) -> Self {
        loop {}
    }
}
impl<K, V> Merge for HashMap<K, V>
where
    K: Clone + Eq + Hash + 'static,
    V: Default + Merge,
{
    type MergeCheck = HashMap<K, <V as Merge>::MergeCheck>;
    fn check_merge<'a>(
        &'a self,
        mut map2: &'a Self,
    ) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(mut self, mut other: Self, mut check: Self::MergeCheck) -> Self {
        loop {}
    }
}
