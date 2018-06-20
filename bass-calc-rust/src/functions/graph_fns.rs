//! Equations from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator"
//! by Douglas H. Hurlburt

use parameters::Parameters;
use num_complex::Complex64;
type C64 = Complex64;

use uom::si::length::{meter};
use uom::si::length;

pub struct BassFnData {
    pub num: Vec<f64>,
    pub den: Vec<f64>
}

fn poly_calc(vec: &Vec<f64>, w: f64) -> C64 {
    vec.iter().rev().enumerate().fold(C64::new(0., 0.), |sum, (index, scale)| {
        let i = C64::new(0.0, w.clone());
        sum + scale * i.powf(index as f64)
    })
}

// Calculate a single point on the graph represented by `data` at frequency `w`
pub fn bass_fn_point(data: &BassFnData, w: f64) -> f64 {
    let num = poly_calc(&data.num, w);
    let den = poly_calc(&data.den, w);
    let n = num / den;
    n.norm_sqr().sqrt()
}

pub fn Radiator(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let g25 = 0.66874030497; // g ^ 0.25
    let α = params.α.v();
    let δ = params.δ.v();
    let psi = α + δ + 1.0;
    let y = params.y.v();
    let y2 = y.sqrt();
    let Qmp = params.Qmp.v();
    let Qs = params.Qs.v();

    let T0 = params.Ts.v() / (y2 * g25); // 8a
    let a1 = (y2 / g25) * 
        ((1.0 / Qmp) + (1.0 / (y * Qs)) + (g * ((α / y) + (y * δ))));

    let a2 = (1.0 / psi.sqrt()) * (((α + 1.0) / y) +
                            (y * (δ + 1.0)) +
                            (1.0 / (Qmp * Qs)) +
                            (g *((α / Qmp) +
                            (y * (δ / Qs)))));

    let a3 = (y2 / psi.powf(0.75)) *
        (((δ + 1.0) / Qs) + ((α + 1.0) / (y * Qmp)) + (g * (α + δ)));
    
    
    
    let b1 = y2 / (Qmp * g25);
    let b2 = y / psi.sqrt();

    BassFnData {
        num: vec![1.0, b1, b2, 0., 0.],
        den: vec![1.0, a1, a2, a3, 1.0]
    }
}

pub fn RadiatorAlt(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let g25 = 0.66874030497; // g ^ 0.25
    let α = params.α.v();
    let δ = params.δ.v();
    let psi = α + δ + 1.0;
    let Ts = params.Ts.v();
    let Ts2 = Ts.powf(2.);
    let Tp = params.Tp.v();
    let Tp2 = Tp.powf(2.);
    let Qmp = params.Qmp.v();
    let Qs = params.Qs.v();

    let b4 = Ts2 * Tp2;
    let b3 = Ts2 * (Tp / Qmp);
    let b2 = Ts2;

    let a4 = Ts2 * Tp2;

    let a3 = Ts2 * Tp / Qmp +
            (g * Ts) * (α * Tp2 + (δ * Ts2));

    let a2 = Tp2 * (α + 1.) +
            Ts2 * (δ + 1.) +
            (Ts * Tp) / (Qs * Qmp) +
            (g * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qs));

    let a1 = Ts * (δ + 1.) / Qs +
            Tp * (α + 1.) / Qmp +
            (g * Ts) * (α + δ);

    BassFnData {
        num: vec![b4, b3, b2, 0., 0.],
        den: vec![a4, a3, a2, a1, psi]
    }
}