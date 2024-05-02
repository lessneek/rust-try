pub trait ToArr<T: Copy, const L: usize> {
    fn copy_to_arr(&self, arr: [T; L]) -> [T; L];
}

impl<T: Copy, const L: usize> ToArr<T, L> for [T] {
    fn copy_to_arr(&self, mut arr: [T; L]) -> [T; L] {
        self.iter()
            .take(L)
            .enumerate()
            .for_each(|(i, item)| arr[i] = *item);
        arr
    }
}

#[test]
#[allow(clippy::useless_vec)]
fn vec_to_arr() {
    assert_eq!(vec![1, 2, 3, 4].copy_to_arr([0i32; 5]), [1, 2, 3, 4, 0]);
    assert_eq!(vec![1, 2, 3, 4, 5].copy_to_arr([0i32; 5]), [1, 2, 3, 4, 5]);
    assert_eq!(vec![1, 2, 3, 4, 5, 6].copy_to_arr([0i32; 5]), [1, 2, 3, 4, 5]);
}

pub trait ArrExt<T: Copy, const L: usize> {
    fn fill_from(&mut self, arr: &[T]);
}

impl<T: Copy, const L: usize> ArrExt<T, L> for [T; L] {
    fn fill_from(&mut self, arr: &[T]) {
        arr.iter()
            .take(L)
            .enumerate()
            .for_each(|(i, item)| self[i] = *item);
    }
}

#[test]
fn arr_fill_from() {
    let mut b = [0i32; 5];
    b.fill_from(&[1, 2, 3, 4]);
    assert_eq!(b, [1, 2, 3, 4, 0]);

    let mut b = [0i32; 5];
    b.fill_from(&[1, 2, 3, 4, 5]);
    assert_eq!(b, [1, 2, 3, 4, 5]);

    let mut b = [0i32; 5];
    b.fill_from(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(b, [1, 2, 3, 4, 5]);
}
