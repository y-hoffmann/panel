use panel::{
    utils::Conversions,
    vec3::Vec3
};

fn main() {
    let tao_a = 0.0.deg_to_rad();
    let tao_d = 180.0.deg_to_rad();

    let epsilon = 23.5.deg_to_rad();
    let gamma = std::f64::consts::PI-52.0.deg_to_rad();

    let delta = 45.0.deg_to_rad();
    let rho = -65.0.deg_to_rad();

    let conf = AngleConfig { epsilon, gamma, delta, rho };

    let n = n(tao_a, tao_d, conf);

    let prod = n*Vec3::from_ex();

    println!("Vector is {}\nDot product is {}", n, prod);
}

#[derive(Clone, Copy)]
struct AngleConfig {
    epsilon: f64,
    gamma: f64,
    delta: f64,
    rho: f64
}

fn n(tao_a: f64, tao_d: f64, conf: AngleConfig) -> Vec3 {
    let e = Vec3::from_ey().rotate(-Vec3::from_ez(), conf.epsilon).rotate(Vec3::from_ey(), tao_a);
    let n0 = Vec3::from_ey().rotate(-Vec3::from_ez(), conf.gamma+conf.epsilon);
    //println!("{}", e);
    Vec3::from_ez().rotate(-Vec3::from_ez(), conf.delta+conf.gamma+conf.epsilon).rotate(n0, conf.rho).rotate(e, tao_d)
}

// fn integrate_dot_prod(daytime: (f64, f64), yeartime: f64, ) -> f64 {
//     for 
// }