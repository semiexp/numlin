use std::ops::{Add, Mul, Sub};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct P(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LP(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D(pub i32, pub i32);

pub const FOUR_NEIGHBOURS: [D; 4] = [D(-1, 0), D(0, -1), D(1, 0), D(0, 1)];

impl LP {
    pub fn of_cell(pos: P) -> LP {
        LP(pos.0 * 2 + 1, pos.1 * 2 + 1)
    }
    pub fn of_vertex(pos: P) -> LP {
        LP(pos.0 * 2, pos.1 * 2)
    }
    pub fn is_edge(self) -> bool {
        self.0 % 2 != self.1 % 2
    }
    pub fn is_vertex(self) -> bool {
        self.0 % 2 == 0 && self.1 % 2 == 0
    }
    pub fn is_cell(self) -> bool {
        self.0 % 2 == 1 && self.1 % 2 == 1
    }
    pub fn as_vertex(self) -> P {
        P(self.0 / 2, self.1 / 2)
    }
    pub fn as_cell(self) -> P {
        P(self.0 / 2, self.1 / 2)
    }
    pub fn y(self) -> i32 {
        self.0
    }
    pub fn x(self) -> i32 {
        self.1
    }
}
impl P {
    pub fn y(self) -> i32 {
        self.0
    }
    pub fn x(self) -> i32 {
        self.1
    }
}
impl D {
    pub fn rotate_clockwise(self) -> D {
        D(self.1, -self.0)
    }
    pub fn rotate_counterclockwise(self) -> D {
        D(-self.1, self.0)
    }
}
impl Add<D> for P {
    type Output = P;
    fn add(self, rhs: D) -> P {
        P(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<D> for P {
    type Output = P;
    fn sub(self, rhs: D) -> P {
        P(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Sub<P> for P {
    type Output = D;
    fn sub(self, rhs: P) -> D {
        D(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Add<D> for LP {
    type Output = LP;
    fn add(self, rhs: D) -> LP {
        LP(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<D> for LP {
    type Output = LP;
    fn sub(self, rhs: D) -> LP {
        LP(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Add<D> for D {
    type Output = D;
    fn add(self, rhs: D) -> D {
        D(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<D> for D {
    type Output = D;
    fn sub(self, rhs: D) -> D {
        D(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Mul<i32> for D {
    type Output = D;
    fn mul(self, rhs: i32) -> D {
        D(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T: Clone> {
    height: i32,
    width: i32,
    data: Vec<T>,
}
impl<T: Clone> Grid<T> {
    pub fn new(height: i32, width: i32, default: T) -> Grid<T> {
        Grid {
            height: height,
            width: width,
            data: vec![default; (height * width) as usize],
        }
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn is_valid_p(&self, pos: P) -> bool {
        0 <= pos.0 && pos.0 < self.height && 0 <= pos.1 && pos.1 < self.width
    }
    pub fn is_valid_lp(&self, pos: LP) -> bool {
        0 <= pos.0 && pos.0 < self.height && 0 <= pos.1 && pos.1 < self.width
    }
    pub fn copy_from(&mut self, src: &Grid<T>)
    where
        T: Copy,
    {
        assert_eq!(self.height, src.height);
        assert_eq!(self.width, src.width);
        self.data.copy_from_slice(&src.data);
    }
    pub fn index_p(&self, pos: P) -> usize {
        (pos.0 * self.width + pos.1) as usize
    }
    pub fn index_lp(&self, pos: LP) -> usize {
        (pos.0 * self.width + pos.1) as usize
    }
    pub fn p(&self, idx: usize) -> P {
        let idx = idx as i32;
        P(idx / self.width, idx % self.width)
    }
    pub fn lp(&self, idx: usize) -> LP {
        let idx = idx as i32;
        LP(idx / self.width, idx % self.width)
    }
}
impl<T: Copy> Grid<T> {
    pub fn get_or_default_p(&self, cd: P, default: T) -> T {
        if self.is_valid_p(cd) {
            self[cd]
        } else {
            default
        }
    }
}
impl<T: Clone> Index<P> for Grid<T> {
    type Output = T;
    fn index<'a>(&'a self, idx: P) -> &'a T {
        let idx = self.index_p(idx);
        &self.data[idx]
    }
}
impl<T: Clone> IndexMut<P> for Grid<T> {
    fn index_mut<'a>(&'a mut self, idx: P) -> &'a mut T {
        let idx = self.index_p(idx);
        &mut self.data[idx]
    }
}
impl<T: Clone> Index<LP> for Grid<T> {
    type Output = T;
    fn index<'a>(&'a self, idx: LP) -> &'a T {
        let idx = self.index_lp(idx);
        &self.data[idx]
    }
}
impl<T: Clone> IndexMut<LP> for Grid<T> {
    fn index_mut<'a>(&'a mut self, idx: LP) -> &'a mut T {
        let idx = self.index_lp(idx);
        &mut self.data[idx]
    }
}
impl<T: Clone> Index<usize> for Grid<T> {
    type Output = T;
    fn index<'a>(&'a self, idx: usize) -> &'a T {
        &self.data[idx]
    }
}
impl<T: Clone> IndexMut<usize> for Grid<T> {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut T {
        &mut self.data[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions() {
        assert_eq!(P(1, 2) + D(3, 0), P(4, 2));
        assert_eq!(P(1, 2) - D(3, 0), P(-2, 2));
        assert_eq!(LP(1, 2) + D(3, 0), LP(4, 2));
        assert_eq!(LP(1, 2) - D(3, 0), LP(-2, 2));
        assert_eq!(D(1, 2) + D(3, 0), D(4, 2));
        assert_eq!(D(1, 2) - D(3, 0), D(-2, 2));
        assert_eq!(D(1, 2) * 4, D(4, 8));

        assert_eq!(D(2, 1).rotate_clockwise(), D(1, -2));
        assert_eq!(D(2, 1).rotate_counterclockwise(), D(-1, 2));

        assert_eq!(LP::of_cell(P(1, 2)), LP(3, 5));
        assert_eq!(LP::of_vertex(P(1, 2)), LP(2, 4));
    }
}
