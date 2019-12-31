use std::convert::From;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use rand::distributions::{Distribution, Standard};

/// A vector with three float components.
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Convenience method to construct a vector.
#[must_use]
#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    /// `Vec3` where all components are zero.
    pub const ZERO: Vec3 = Vec3::all(0.0);

    /// `Vec3` where all components are one.
    pub const ONE: Vec3 = Vec3::all(1.0);

    /// Construct a new `Vec3` with three float components.
    ///
    /// # Example
    /// ```rust
    /// Vec3::new(1.0, 2.0, 3.0)
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Construct a new `Vec3` with three float components
    /// all being the same initial value.
    ///
    /// # Example
    /// ```rust
    /// Vec3::all(1.0) == Vec3::ONE
    /// ```
    #[must_use]
    #[inline]
    pub const fn all(f: f32) -> Self {
        Self::new(f, f, f)
    }

    /// Returns the magnitude of the vector, computed using
    /// the pythagorean theorem.
    /// 
    /// # Example
    /// ```rust
    /// Vec3::new(3.0, 4.0, 0.0).mag() == 5.0
    /// ```
    #[must_use]
    #[inline]
    pub fn mag(self) -> f32 {
        self.mag_sq().sqrt()
    }

    /// Returns the squared magnitude of the vector, computed
    /// using the pythagorean theorem.
    /// 
    /// # Example
    /// ```rust
    /// Vec3::new(3.0, 4.0, 0.0).mag_sq() == 25.0
    /// ```
    #[must_use]
    #[inline]
    pub fn mag_sq(self) -> f32 {
        self.dot(self)
    }

    /// Returns a new vector where a mapping function is applied to
    /// all of the components of the previous vector.
    ///
    /// # Example
    /// ```rust
    /// Vec3::new(9.0, 16.0, 25.0).map(f32::sqrt) == Vec3::new(3.0, 4.0, 5.0)
    /// ```
    #[must_use]
    #[inline]
    pub fn map(self, f: impl Fn(f32) -> f32) -> Self {
        Self::new(f(self.x), f(self.y), f(self.z))
    }

    /// Returns a new normalized vector where the components of the vector
    /// is scaled so that the magnitude is one, aka. a unit vector.
    /// 
    /// # Example
    /// ```rust
    /// Vec3::new(20.0, 0.0, 0.0).normalize() == Vec3::new(1.0, 0.0, 0.0)
    /// 
    /// dbg!(Vec3::ONE.normalize())
    /// // Outputs Vec3 { x: 0.57735026, y: 0.57735026, z: 0.57735026 }
    /// ```
    #[must_use]
    #[inline]
    pub fn normalize(self) -> Self {
        self / self.mag()
    }

    /// Returns the result of the dot product between this vector and
    /// the `rhs` argument vector.
    #[must_use]
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Returns the result of the cross product between this vector
    /// and the `rhs` argument vector.
    #[must_use]
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Returns a reflected direction given a normal direction on the object.
    ///
    /// # Assumptions
    /// The `self` and the `normal` vector represents directions.
    #[must_use]
    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - self.dot(normal) * normal * 2.0
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(tuple: (f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(vec: Vec3) -> Self {
        (vec.x, vec.y, vec.z)
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        let u = rng.gen::<f32>();
        let v = rng.gen::<f32>();
        let theta = u * 2.0 * std::f32::consts::PI;
        let phi = (2.0 * v - 1.0).acos();
        let r = rng.gen::<f32>().cbrt();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let x = r * sin_phi * cos_theta;
        let y = r * sin_phi * sin_theta;
        let z = r * cos_phi;
        vec3(x, y, z)
    }
}
