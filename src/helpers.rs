/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::{
    fmt::Display,
    ops::{Deref, DerefMut, Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct Grid<T>(pub Vec<Vec<T>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn from_x_y((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
    pub fn ensure_between(
        &mut self,
        (x_min, x_max): (isize, isize),
        (y_min, y_max): (isize, isize),
    ) {
        if self.x < x_min {
            self.x += x_max;
        } else if self.x > x_max {
            self.x -= x_max;
        }
        if self.y < y_min {
            self.y += y_max;
        } else if self.y > y_max {
            self.y -= y_max;
        }
    }

    pub fn manhattan_distance_to(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn neighbors_of(&self, p: Point) -> Vec<Point> {
        let mut neighbors = vec![
            Point::from_x_y((p.x - 1, p.y)),
            Point::from_x_y((p.x + 1, p.y)),
            Point::from_x_y((p.x, p.y - 1)),
            Point::from_x_y((p.x, p.y + 1)),
        ];
        neighbors.retain(|p| {
            p.x >= 0 && p.y >= 0 && p.x < self.width() as isize && p.y < self.height() as isize
        });
        neighbors
    }
}

impl<T> PartialEq for Grid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Deref for Grid<T> {
    fn deref(&self) -> &Vec<Vec<T>> {
        &self.0
    }
    type Target = Vec<Vec<T>>;
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.0
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
