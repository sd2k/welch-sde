use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rand_distr::StandardNormal;
use welch_sde::{Build, SpectralDensity};

fn spectral_density(c: &mut Criterion) {
    let n = 1e5 as usize;
    let fs = 10e3;
    let amp = 2. * 2f64.sqrt();
    let freq = 1550f64;
    let noise_power = 0.001 * fs / 2.;
    let signal: Vec<f64> = (0..n)
        .map(|i| i as f64 / fs)
        .map(|t| {
            amp * (2. * std::f64::consts::PI * freq * t).sin()
                + thread_rng().sample::<f64, StandardNormal>(StandardNormal) * noise_power.sqrt()
        })
        .collect();
    let welch: SpectralDensity<f64> = SpectralDensity::builder(&signal, fs).build();

    c.bench_function("periodogram", |b| {
        b.iter(|| {
            welch.periodogram();
        });
    });
}

criterion_group!(benches, spectral_density);
criterion_main!(benches);
