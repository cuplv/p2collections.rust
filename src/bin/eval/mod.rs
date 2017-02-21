pub mod actions;
pub mod eval_iraz;
pub mod eval_vec;
pub mod seq_test;
pub mod types;

use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use rand::{Rand, Rng, StdRng};
use time::Duration;

/// convenience traits for incremental test data
pub trait Adapt: 'static+Eq+Clone+Hash+Debug {}
impl<E> Adapt for E where E: 'static+Eq+Clone+Hash+Debug {}
pub trait Eval: Adapt+Rand {}
impl<E> Eval for E where E: Adapt+Rand {}

////////////////////////////////////
// primitive traits
// optional and more can be included
////////////////////////////////////

/// for building an incremental collection
pub trait CreateInc<G:Rng> {
	fn inc_init(size: usize, unitgauge: usize, namegauge: usize, coord: &G, rng: &mut StdRng) -> (Duration,Self);
}
/// for adding elements as if initialization was longer
pub trait EditExtend {
	fn extend(self, batch_size: usize, rng: &mut StdRng) -> (Duration,Self);
}
/// for adding elements as if the user is editing
pub trait EditAppend {
	fn append(self, batch_size: usize, rng: &mut StdRng) -> (Duration,Self);
}
/// for inserting elements at random location
pub trait EditInsert {
	fn insert(self, batch_size: usize, rng: &mut StdRng) -> (Duration,Self);
}
/// for computing the max of the collection
pub trait CompMax {
	type Target;
	fn comp_max(&self, rng: &mut StdRng) -> (Duration,Self::Target);
}

pub trait CompTreeFold<R,O,I:Fn(&R)->O,B:Fn(O,O)->O> {
	type Target;
	fn comp_tfold(&self, init:Rc<I>, bin:Rc<B>, rng: &mut StdRng) -> (Duration,Self::Target);
}

/// changes every value to another based on function
pub trait CompMap<I,O,F:Fn(&I)->O> {
	type Target;
	fn comp_map(&self, f:Rc<F>, rng: &mut StdRng) -> (Duration,Self::Target);
}

/// folds every element into the binary function, starting with the given one
pub trait CompFold<I,O,F:Fn(O,&I)->O> {
	type Target;
	fn comp_fold(&self, accum: O, f:Rc<F>, rng: &mut StdRng) -> (Duration,Self::Target);
}

////////////////////////////////
// Types of actions
// limited number, unlimited use
////////////////////////////////

pub trait Creator<R,D> {
	fn create(&mut self, rnd: &mut StdRng) -> (R,D);
}
pub trait Editor<R,D> {
	fn edit(&mut self, data: D, rng: &mut StdRng) -> (R,D);
}
pub trait Computor<R,D> {
	fn compute(&mut self, data: &D, rng: &mut StdRng) -> R;
}

/// Test framework
pub trait Testor<R> {
	fn test(&mut self, rng: &mut StdRng) -> R;
}
