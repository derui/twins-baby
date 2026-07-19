//# Tag module

/// A macro to add From<u64> for index
///
/// use like:
/// ```index_impl(SampleIndex)```
macro_rules! tag_impl {
    ($ty:ident) => {
        impl $ty {
            pub fn new(id: u64) -> Self {
                $ty(id)
            }
        }

        impl From<u64> for $ty {
            fn from(id: u64) -> Self {
                $ty(id)
            }
        }

        impl From<$ty> for u64 {
            fn from(id: $ty) -> Self {
                id.0
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", stringify!($ty), self.0)
            }
        }
    };
}

/// A tag is a unique identifier for an entity in the CAD model. It is used to reference entities in the model and to store metadata about them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceTag(u64);
tag_impl!(FaceTag);

/// A tag is a unique identifier for a solid in the CAD model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SolidTag(u64);
tag_impl!(SolidTag);
