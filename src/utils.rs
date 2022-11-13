use num::Num;

pub fn linmap<T>(val: T, range_a: (T, T), range_b: (T, T)) -> T
where
    T: Num + Copy,
{
    let (lower_a, upper_a) = range_a;
    let (lower_b, upper_b) = range_b;
    ((val - lower_a) / (upper_a - lower_a)) * (upper_b - lower_b) + lower_b
}
