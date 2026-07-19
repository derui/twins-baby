/// A trait for resolving references to a specific type.
pub trait Resolve<'a, R, S> {
    type Output;

    /// Resolves a reference of type R to a value of type Output.
    fn resolve(&'a self, ref_: R) -> Option<Self::Output>;
}
