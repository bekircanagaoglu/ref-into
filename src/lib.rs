pub trait RefInto<T>: Sized {
    fn ref_into(&self) -> T;
}