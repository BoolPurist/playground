use iter_num_tools::arange;
extern crate nalgebra as na;

pub type Num = f64;
pub type Point2D = (f64, f64);
pub type Point = Point2D;
pub type Func2D = dyn Fn(Num) -> Num;
pub type Func = Func2D;

pub fn plot<'a, F: 'a + Fn(Num) -> Num>(
    f: F,
    min: Num,
    max: Num,
    step: Num,
) -> impl 'a + Iterator<Item = (Num, Num)> {
    let range = min..max;
    arange(range, step).map(move |x| (x, f(x)))
}
// TODO div0 and such: return option
