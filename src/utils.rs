/// https://docs.rs/rulinalg/0.3.0/src/rulinalg/.cargo/registry/src/github.com-1ecc6299db9ec823/rulinalg-0.3.0/src/utils.rs.html#243-257
pub(crate) fn argmax<T: Copy + PartialOrd>(u: &[T]) -> (usize, T) {
    assert!(u.len() != 0);

    let mut max_index = 0;
    let mut max = u[max_index];

    for (i, v) in (u.iter()).enumerate() {
        if max < *v {
            max_index = i;
            max = *v;
        }
    }

    (max_index, max)
}

pub(crate) fn argmin<T: Copy + PartialOrd>(u: &[T]) -> (usize, T) {
    assert!(u.len() != 0);

    let mut min_index = 0;
    let mut min = u[min_index];

    for (i, v) in (u.iter()).enumerate() {
        if min > *v {
            min_index = i;
            min = *v;
        }
    }

    (min_index, min)
}
