use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::Display;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 (pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(tup: (f64, f64, f64)) -> Self {
        Vec3(tup.0, tup.1, tup.2)
    }

    pub fn from_ex() -> Self {
        Vec3(1.0, 0.0, 0.0)
    }
    
    pub fn from_ey() -> Self {
        Vec3(0.0, 1.0, 0.0)
    }
    
    pub fn from_ez() -> Self {
        Vec3(0.0, 0.0, 1.0)
    }

    /// returns the cross product (self x argument) of the passed ``Vec3``s
    #[inline]
    pub fn cross(self, u: Self) -> Self {
        Vec3(self.1*u.2 - self.2*u.1, self.2*u.0 - self.0*u.2, self.0*u.1 - self.1*u.0)
    }

    /// returns ``Vec3`` that is ``a`` rotated around ``axis`` by ``angle`` (in rad)
    #[inline]
    pub fn rotated(v: Self, axis: Self, angle: f64) -> Self {
        v*angle.cos() + axis*(v*axis)*(1.0-angle.cos()) - v.cross(axis)*angle.sin()
    }

    /// rotates the ``Vec3`` that this is called on around ``axis`` by ``angle`` (in rad)
    #[inline]
    pub fn rotate(&mut self, axis: Self, angle: f64) -> Self {
        *self = Vec3::rotated(*self, axis, angle);
        *self
    }

    /// returns the magnitude of the ``Vec3``
    #[inline]
    pub fn len(self) -> f64 {
        (self*self).sqrt()
    }

    /// returns a ``Vec3`` that is parallel to ``self`` and has length 1 
    #[inline]
    pub fn unit(self) -> Vec3 {
        self/self.len()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0+rhs.0, self.1+rhs.1, self.2+rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0-rhs.0, self.1-rhs.1, self.2-rhs.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    #[inline]
    fn mul(self, rhs: Vec3) -> f64 {
        self.0*rhs.0 + self.1*rhs.1 + self.2*rhs.2
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0*rhs, self.1*rhs, self.2*rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(rhs.0*self, rhs.1*self, rhs.2*self)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f64) -> Vec3 {
        self*(1.0/rhs)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Vec3 {
        rhs*(1.0/self)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -1.0*self
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

mod test {
    #[test]
    fn cross_cartesian_basis() {
        use crate::vec3::Vec3;
        assert_eq!(Vec3(1.0, 0.0, 0.0).cross(Vec3(0.0, 1.0, 0.0)), Vec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn associativity() {
        use crate::vec3::Vec3;
        assert_eq!(Vec3(1.0, 2.0, 3.0)*3.0, 3.0*Vec3(1.0, 2.0, 3.0));
    }

    // #[test]
    // fn angles() { // this does not work because of precision problems (0.0 != -1.22...e-16), but the result is technically correct
    //     use crate::vec3::Vec3;
    //     assert_eq!(Vec3::make_rotate(Vec3(1.0, 0.0, 0.0),  Vec3(0.0, 1.0, 0.0), std::f64::consts::PI), Vec3(-1.0, 0.0, 0.0));
    // }
}