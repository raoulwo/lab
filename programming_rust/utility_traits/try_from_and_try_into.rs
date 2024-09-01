// `TryFrom` and `TryInto` are like `From` and
// `Into` for conversions that might fail. Their
// traits look like this:
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

fn main() {}
