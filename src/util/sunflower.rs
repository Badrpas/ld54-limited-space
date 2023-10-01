pub fn sunflower(k: usize, n: usize, alpha: f32, length: f32) -> (f32, f32) {
    let b = (alpha * (n as f32).sqrt()).round();
    let phi: f32 = ((5. as f32).sqrt() + 1.) / 2.;
    let phi_2: f32 = phi * phi;
    let k = k as f32;
    let r = radius(k, n as f32, b) * length;
    let theta = 2. * std::f32::consts::PI * k / phi_2;
    let x = r * theta.cos();
    let y = r * theta.sin();
    (x, y)
}

fn radius(k: f32, n: f32, b: f32) -> f32 {
    if k > n - b {
        1.
    } else {
        (k - 1. / 2.).sqrt() / (n - (b + 1.) / 2.).sqrt()
    }
}

