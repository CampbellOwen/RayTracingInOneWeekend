use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    u.cross(v)
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.dot(v)
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, _rhs: f32) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f32) -> Self {
        Self {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, _rhs: f32) {
        self.x /= _rhs;
        self.y /= _rhs;
        self.z /= _rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::{cross, dot, Vec3};
    #[test]
    fn test_dot() {
        assert_eq!(
            dot(
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                }
            ),
            1.0
        );
        assert_eq!(
            dot(
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0
                }
            ),
            0.0
        );
        assert_eq!(
            dot(
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0
                }
            ),
            0.0
        );
        assert_eq!(
            dot(
                &Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0
                }
            ),
            0.0
        );
    }

    #[test]
    pub fn test_cross() {
        assert_eq!(
            cross(
                &Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                }
            ),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0
            }
        );
    }
}