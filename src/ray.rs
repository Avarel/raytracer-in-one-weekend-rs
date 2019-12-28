use crate::vec3::Vec3;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T> Ray<T> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Self { origin, direction }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Mul<T, Output = T> + Copy> Ray<T> {
    #[inline]
    pub fn point_at_parameter(self, parameter: T) -> Vec3<T> {
        self.origin + self.direction * parameter
    }
}