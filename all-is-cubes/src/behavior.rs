//! Dynamic add-ons to game objects; we might also have called them “components”.
use crate::transaction::{self, Transaction};
use crate::universe::VisitRefs;
use downcast_rs::{impl_downcast, Downcast};
use std::fmt::{self, Debug};
impl_downcast!(Behavior < H > where H : BehaviorHost);
