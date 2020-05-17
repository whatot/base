use std::ops::Mul;

trait HasArea<T> {
    fn area(&self) -> T;
}

struct Rectangle<T> {
    longth: T,
    wide: T,
}

impl<T> HasArea<T> for Rectangle<T>
where
    T: Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.longth * self.wide
    }
}

#[test]
fn test_area_overload_operator() {
    let rec = Rectangle {
        longth: 3.0f64,
        wide: 4.0f64,
    };

    assert_eq!(12f64, rec.area());
}
