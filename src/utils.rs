pub trait Conversions {
    fn deg_to_rad(self) -> Self;
    
    fn rad_to_deg(self) -> Self;
}

pub const DEG_TO_RAD: f64 = 1.0/180.0*std::f64::consts::PI;
pub const RAD_TO_DEG: f64 = 1.0/DEG_TO_RAD;

impl Conversions for f64 {
    #[inline]
    fn deg_to_rad(self) -> Self {
        self*DEG_TO_RAD
    }

    #[inline]
    fn rad_to_deg(self) -> Self {
        self*RAD_TO_DEG
    }
}