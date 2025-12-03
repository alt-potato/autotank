use fastnum::{D64, dec64};
use serde::{Deserialize, Serialize};

/// A scalar (one-dimensional) value.
pub type Scalar = D64;

pub trait ConvertToScalar: Sized {
    fn to_scalar(self) -> Scalar;
}

impl ConvertToScalar for f64 {
    fn to_scalar(self) -> Scalar {
        D64::from_f64(self)
    }
}

impl ConvertToScalar for u32 {
    fn to_scalar(self) -> Scalar {
        D64::from_u32(self)
    }
}

/// A two-dimensional vector.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2 {
    /// Returns a zero vector.
    pub fn zero() -> Vec2 {
        Vec2::new(dec64!(0), dec64!(0))
    }

    /// Creates a new vector with the given x and y components.
    pub fn new(x: Scalar, y: Scalar) -> Vec2 {
        Vec2 { x, y }
    }

    /// Creates a new vector with the given x and y components from f64 values.
    pub fn new_from_f64(x: f64, y: f64) -> Vec2 {
        Vec2::new(x.to_scalar(), y.to_scalar())
    }

    /// Creates a new vector from the given (r, theta) pair.
    pub fn new_from_angle(magnitude: Scalar, angle: Scalar) -> Vec2 {
        Vec2::new(magnitude * angle.cos(), magnitude * angle.sin())
    }

    /// Computes the sum of two vectors.
    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    /// Computes the difference of two vectors.
    pub fn sub(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: &Vec2) -> Scalar {
        self.x * other.x + self.y * other.y
    }

    /// Computes the cross product of two vectors.
    pub fn cross(&self, other: Vec2) -> Scalar {
        self.x * other.y - self.y * other.x
    }

    /// Computes the square of the length of the vector.
    pub fn length_squared(&self) -> Scalar {
        self.dot(self)
    }

    /// Rotates the vector by the given angle, in radians.
    pub fn rotate(&self, angle: Scalar) -> Vec2 {
        Vec2::new(
            self.x * angle.cos() - self.y * angle.sin(),
            self.x * angle.sin() + self.y * angle.cos(),
        )
    }

    /// Normalizes the vector, returning a unit vector.
    pub fn normalize(&self) -> Vec2 {
        let length = self.length_squared().sqrt();
        Vec2::new(self.x / length, self.y / length)
    }

    /// Converts the vector to polar coordinates (r, theta).
    pub fn to_polar(&self) -> (Scalar, Scalar) {
        (self.length_squared().sqrt(), self.y.atan2(self.x))
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_new_should_create_vector_with_correct_components() {
        // Arrange
        let x = 1.0.to_scalar();
        let y = 2.0.to_scalar();

        // Act
        let v = Vec2::new(x, y);

        // Assert
        assert_eq!(v.x, x);
        assert_eq!(v.y, y);
    }

    #[test]
    fn vec2_zero_should_return_zero_vector() {
        // Arrange & Act
        let v = Vec2::zero();

        // Assert
        assert_eq!(v.x, 0.0.to_scalar());
        assert_eq!(v.y, 0.0.to_scalar());
    }

    #[test]
    fn vec2_new_from_f64_should_create_vector_with_correct_components() {
        // Arrange & Act
        let v = Vec2::new_from_f64(1.0, 2.0);

        // Assert
        assert_eq!(v.x, 1.0.to_scalar());
        assert_eq!(v.y, 2.0.to_scalar());
    }

    #[test]
    fn vec2_new_from_angle_should_create_vector_from_polar_coordinates() {
        // Arrange
        let magnitude = 1.0.to_scalar();
        let angle = 0.0.to_scalar(); // 0 radians

        // Act
        let v = Vec2::new_from_angle(magnitude, angle);

        // Assert
        assert_eq!(v.x, 1.0.to_scalar()); // cos(0) = 1
        assert_eq!(v.y, 0.0.to_scalar()); // sin(0) = 0

        // Arrange for second case
        let angle_pi_half = Scalar::PI / 2.0.to_scalar(); // pi/2 radians

        // Act
        let v_pi_half = Vec2::new_from_angle(magnitude, angle_pi_half);

        // Assert
        // D64 math is deterministic, but subject to the precision of PI.
        // The results for cos(PI/2) and sin(PI/2) will be very close but not exactly 0 and 1.
        assert_eq!(v_pi_half.x, angle_pi_half.cos());
        assert_eq!(v_pi_half.y, angle_pi_half.sin());
    }

    #[test]
    fn vec2_dot_should_compute_dot_product() {
        // Arrange
        let v1 = Vec2::new_from_f64(1.0, 2.0);
        let v2 = Vec2::new_from_f64(3.0, 4.0);

        // Act
        let result = v1.dot(&v2);

        // Assert
        assert_eq!(result, (1.0 * 3.0 + 2.0 * 4.0).to_scalar()); // 3 + 8 = 11
    }

    #[test]
    fn vec2_cross_should_compute_cross_product() {
        // Arrange
        let v1 = Vec2::new_from_f64(1.0, 0.0);
        let v2 = Vec2::new_from_f64(0.0, 1.0);

        // Act
        let result = v1.cross(v2);

        // Assert
        assert_eq!(result, 1.0.to_scalar()); // 1*1 - 0*0 = 1
    }

    #[test]
    fn vec2_length_squared_should_compute_squared_length() {
        // Arrange
        let v = Vec2::new_from_f64(3.0, 4.0);

        // Act
        let result = v.length_squared();

        // Assert
        assert_eq!(result, (3.0 * 3.0 + 4.0 * 4.0).to_scalar()); // 9 + 16 = 25
    }

    #[test]
    fn vec2_rotate_should_rotate_vector_by_angle() {
        // Arrange
        let v = Vec2::new_from_f64(1.0, 0.0);
        let angle_90 = Scalar::PI / 2.0.to_scalar();
        let angle_180 = Scalar::PI;

        // Act
        let rotated_90 = v.rotate(angle_90);

        // Assert
        assert_eq!(rotated_90.x, angle_90.cos());
        assert_eq!(rotated_90.y, angle_90.sin());

        // Act for second case
        let rotated_180 = v.rotate(angle_180);

        // Assert
        assert_eq!(rotated_180.x, angle_180.cos());
        assert_eq!(rotated_180.y, angle_180.sin());
    }

    #[test]
    fn vec2_normalize_should_return_unit_vector() {
        // Arrange
        let v = Vec2::new_from_f64(3.0, 4.0);

        // Act
        let normalized = v.normalize();

        // Assert
        let expected_x = 3.0.to_scalar() / 5.0.to_scalar();
        let expected_y = 4.0.to_scalar() / 5.0.to_scalar();
        assert_eq!(normalized.x, expected_x);
        assert_eq!(normalized.y, expected_y);
        assert_eq!(normalized.length_squared(), 1.0.to_scalar());
    }

    #[test]
    fn vec2_to_polar_should_convert_to_polar_coordinates() {
        // Arrange
        let v = Vec2::new_from_f64(1.0, 1.0);

        // Act
        let (magnitude, angle) = v.to_polar();

        // Assert
        assert_eq!(magnitude, 2.0.to_scalar().sqrt());
        // For a 45-degree angle, cos(angle) must equal sin(angle).
        // This avoids comparing two different calculations of PI/4 which may have precision differences.
        assert_eq!(angle.cos(), angle.sin());

        // Arrange for second case
        let v2 = Vec2::new_from_f64(-1.0, 0.0);

        // Act
        let (magnitude2, angle2) = v2.to_polar();

        // Assert
        assert_eq!(magnitude2, 1.0.to_scalar());
        assert_eq!(angle2, Scalar::PI);
    }
}
