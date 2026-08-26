//@ revisions: current next
//@[next] compile-flags: -Znext-solver
// Regression test for #161404: generalization incorrectly handled higher-ranked aliases

#![forbid(unsafe_code)]

struct Wrap<T>(T);

trait Id {
    type Out;
}
impl<T> Id for T {
    type Out = u16;
}

trait Probe {}
impl<'c> Probe for &'c Wrap<for<'a> fn(<&'a &'c u64 as Id>::Out)> {}

trait Indirect {}
impl<P: Probe> Indirect for P {}

trait Mark {}
impl Mark for fn(u16) {}

trait Select {
    type Assoc;
}

impl<X> Select for X
where
    for<'d> &'d Wrap<X>: Indirect,
{
    type Assoc = usize;
}

impl<Z: Mark> Select for Z {
    //~^ ERROR conflicting implementations of trait `Select` for type `fn(u16)`
    type Assoc = &'static i32;
}

fn main() {}
