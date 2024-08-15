//// chirp generation

use std::f32::consts;

pub fn linear_chirp(begin: f32, end: f32, samples: usize, rate: u32) -> Vec<f32> {
    assert!(
        begin.is_sign_positive(),
        "Starting frequency of linear chirp cannot be negative!"
    );

    // initialize vector with needed size (no reallocation needed)
    let mut signal = vec![0.0; samples];
    // time step
    let dt: f32 = 1.0 / rate as f32;
    // phase (argument of cosine)
    let mut phi: f32 = 0.0;
    // angular frequency dphi/dt (rate of change of phase, aka phi')
    let mut dphidt: f32 = consts::TAU * begin;
    // frequency delta is second derivative with constant dt premultiplied
    let ddphi: f32 = consts::TAU * (end - begin) / samples as f32;

    // reassign the samples to actual values
    for sample in signal.iter_mut() {
        // calculate cosine amplitude based on current phi
        *sample = phi.cos();
        // phase delta; phi' multiplied with dt timestep
        phi += dt * dphidt;
        // phi" and dt are constant, so we use the premultiplied value
        dphidt += ddphi;
        // may help with accuracy, but might not be necessary
        if phi > consts::TAU {
            phi -= consts::TAU;
        }
        if -phi > consts::TAU {
            phi += consts::TAU;
        }
    }

    signal
}

pub fn exponential_chirp(begin: f32, end: f32, samples: usize, rate: u32) -> Vec<f32> {
    assert!(
        begin.is_sign_positive(),
        "Starting frequency of exponential chirp cannot be negative!"
    );
    assert!(
        begin > 0.0f32,
        "Starting frequency of exponential chirp cannot be zero"
    );

    // initialize vector with needed size (no reallocation needed)
    let mut signal = vec![0.0; samples];
    // time step
    let dt: f32 = 1.0 / rate as f32;
    // phase (argument of cosine)
    let mut phi: f32 = 0.0;
    // exponent term (log of angular frequency)
    let mut x: f32 = (consts::TAU * begin).log2();
    // delta of exponent term (dx/dt with premultiplied time step)
    let dx: f32 = dt * (end / begin).log2();

    // reassign the samples to actual values
    for sample in signal.iter_mut() {
        // calculate cosine amplitude based on current phi
        *sample = phi.cos();
        // phase delta; phi' multiplied with dt timestep
        phi += dt * x.exp2();
        // x' and dt are constant, so we use the premultiplied value
        x += dx;
        // may help with accuracy, but might not be necessary
        if phi > consts::TAU {
            phi -= consts::TAU;
        }
        if -phi > consts::TAU {
            phi += consts::TAU;
        }
    }

    signal
}

//// writing

use wavers::{self, WaversResult};

pub fn write_to_wav(filepath: &str, signal: &Vec<f32>, rate: u32) -> WaversResult<()> {
    wavers::write(filepath, signal, rate as i32, 1) // our signals are always mono
}
