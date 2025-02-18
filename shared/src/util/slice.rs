pub trait SliceExt<T> {
    fn split_off_first_at(&mut self, mid: usize) -> &[T];
}

impl<T> SliceExt<T> for &[T] {
    fn split_off_first_at(&mut self, mid: usize) -> &[T] {
        let (first, second) = self.split_at(mid);
        *self = second;
        first
    }
}
