pub trait FromIter {
    fn from_iter(iter: impl Iterator<Item = (String, String)>) -> Option<Self>
    where
        Self: Sized;
}
