use nalgebra::{Vector2, Matrix2};

pub struct SIE {
    einstein_radius: f64,
    axis_ratio: f64,
    orientation: f64,
}

impl SIE {
    pub fn new(einstein_radius: f64, axis_ratio: f64, orientation: f64) -> Self {
        Self {
            einstein_radius,
            axis_ratio,
            orientation,
        }
    }

    pub fn deflection_angle(&self, theta: Vector2<f64>) -> Vector2<f64> {
        let phi = theta.y.atan2(theta.x); // Calculate the angle directly using atan2
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        let q = self.axis_ratio;
        let theta_r = theta.norm() / self.einstein_radius;

        let qcos2 = q * q * cos_phi * cos_phi;
        let qsin2 = q * q * sin_phi * sin_phi;
        let q2 = q * q;
        let theta_rcos2 = theta_r * (1.0 - qcos2);
        let theta_rsin2 = theta_r * (1.0 - qsin2);

        let mut deflection = Vector2::new(0.0, 0.0);
        deflection.x = theta_r * (1.0 - q2) / theta_rcos2.sqrt();
        deflection.y = theta_r * (1.0 + q2) / theta_rsin2.sqrt();
        deflection.x *= if cos_phi >= 0.0 { 1.0 } else { -1.0 };
        deflection.y *= if sin_phi >= 0.0 { 1.0 } else { -1.0 };

        // Rotate the deflection angle according to the orientation
        let rot_matrix = Matrix2::new(self.orientation.cos(), -self.orientation.sin(),
                                      self.orientation.sin(), self.orientation.cos());
        deflection = rot_matrix * deflection;

        deflection
    }
}
