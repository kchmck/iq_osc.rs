//! Given
//!
//! > f(t) = cos(θ<sub>0</sub> + ωt) = cos Φ(t),
//!
//! an oscillator is defined here to evaluate f(0), f(1), f(2), ... in sequence to
//! generate a sinusoidal signal. Further, a *quadrature* oscillator also evaluates
//! g(t) = sin Φ(t) at each step for the quadrature signal.
//!
//! Calling out to these trig functions at each evaluation of f(t) and g(t) can be costly
//! with a high sample rate or within a tight loop. As an alternative, this crate
//! implements a quadrature oscillator that replaces the 2 trig function calls at each
//! evaluation with 6 arithmetic operations (4 multiplies, 1 addition, and 1 subtraction.)
//!
//! ## Theory
//!
//! Using the definition of f(t) = cos Φ(t) from above, notice that
//!
//! > Φ(0) = θ<sub>0</sub>
//! >
//! > Φ(1) = θ<sub>0</sub> + ω = Φ(0) + ω
//! >
//! > Φ(2) = θ<sub>0</sub> + 2ω = Φ(1) + ω
//!
//! And in general,
//!
//! > Φ(t) = Φ(t - 1) + ω, t > 0
//!
//! With this and the trig identities
//!
//! > cos(u + v) = cos(u)cos(v) - sin(u)sin(v)
//! >
//! > sin(u + v) = sin(u)cos(v) + cos(u)sin(v)
//!
//! we can then write f(t) as
//!
//! > f(t) = cos(Φ(t - 1) + ω) = cos(Φ(t - 1))cos(ω) - sin(Φ(t - 1))sin(ω)
//!
//! and similarly for the quadrature signal,
//!
//! > g(t) = sin(Φ(t - 1) + ω) = sin(Φ(t - 1))cos(ω) + cos(Φ(t - 1))sin(ω)
//!
//! If we compute sin ω, cos ω, sin θ<sub>0</sub>, and cos θ<sub>0</sub> at
//! initialization, then we can compute f(t) and g(t) for t = 0, 1, 2, ... using just the
//! arithmetic in the above equations.
//!
//! ## Error Accumulation
//!
//! Due to the accumulation of floating-point roundoff errors, the accuracy of returned
//! sine/cosine evaulations will slowly degrade over phase steps. Using a very small phase
//! step or running a `QuadOsc` through many, many cycles will make this problem more
//! pronounced. As a workaround, the double-precision `QuadOsc<f64>` can be used, which
//! provides a significant increase in accuracy across phase steps and has relatively
//! little impact on speed (compare `bench_osc32` and `bench_osc64` in the output of
//! `cargo bench`.)

extern crate num;

use num::Float;

/// Quadrature oscillator with current phase Φ(t) and phase step ω.
pub struct QuadOsc<T: Float> {
    /// Holds (sin ω, cos ω) for the phase step ω.
    step: (T, T),
    /// Holds (sin Φ(t), cos Φ(t)) for the current phase Φ(t).
    phase: (T, T),
}

impl<T: Float> QuadOsc<T> {
    /// Create a new `QuadOsc` starting at the given initial phase θ<sub>0</sub> (in
    /// radians) and with the given phase step ω (in radians).
    ///
    /// The first call to `next()` will then return (sin θ<sub>0</sub>, cos
    /// θ<sub>0</sub>).
    pub fn new(phase: T, step: T) -> Self {
        QuadOsc {
            step: step.sin_cos(),
            phase: phase.sin_cos(),
        }
    }

    /// Change the phase step to the given ω (in radians).
    pub fn set_step(&mut self, step: T) {
        self.step = step.sin_cos();
    }

    /// Step the phase to Φ(t+1) and return (sin Φ(t), cos Φ(t)).
    pub fn next(&mut self) -> (T, T) {
        let cur = self.phase;

        self.phase = (
            // Compute sin(Φ(t - 1))cos(ω) + cos(Φ(t - w))sin(ω).
            self.phase.0 * self.step.1 + self.phase.1 * self.step.0,
            // Compute cos(Φ(t - 1))cos(ω) - sin(Φ(t - 1))sin(ω).
            self.phase.1 * self.step.1 - self.phase.0 * self.step.0,
        );

        cur
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::PI as PI32;
    use std::f64::consts::PI as PI64;

    #[test]
    fn test_osc() {
        let mut o = QuadOsc::new(0.0, PI32 / 2.0);

        for _ in 0..100 {
            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 0.0001);
            assert!((cos - 1.0).abs() < 0.0001);

            let (sin, cos) = o.next();
            assert!((sin - 1.0).abs() < 0.0001);
            assert!((cos - 0.0).abs() < 0.0001);

            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 0.0001);
            assert!((cos - -1.0).abs() < 0.0001);

            let (sin, cos) = o.next();
            assert!((sin - -1.0).abs() < 0.0001);
            assert!((cos - 0.0).abs() < 0.0001);
        }
    }

    #[test]
    fn test_err_32() {
        // Tests single precision error accumulation over a relatively large number of
        // iterations.

        let mut o = QuadOsc::new(0.0, PI32 / 20.0);

        for _ in 0..1024 {
            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 1.0e-3);
            assert!((cos - 1.0).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.15643446504023087).abs() < 1.0e-3);
            assert!((cos - 0.98768834059513777).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.3090169943749474).abs() < 1.0e-3);
            assert!((cos - 0.95105651629515353).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.45399049973954675).abs() < 1.0e-3);
            assert!((cos - 0.8910065241883679).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.58778525229247314).abs() < 1.0e-3);
            assert!((cos - 0.80901699437494745).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.70710678118654746).abs() < 1.0e-3);
            assert!((cos - 0.70710678118654757).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.80901699437494745).abs() < 1.0e-3);
            assert!((cos - 0.58778525229247314).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.89100652418836779).abs() < 1.0e-3);
            assert!((cos - 0.4539904997395468).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.95105651629515353).abs() < 1.0e-3);
            assert!((cos - 0.30901699437494745).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.98768834059513777).abs() < 1.0e-3);
            assert!((cos - 0.15643446504023092).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 1.0).abs() < 1.0e-3);
            assert!((cos - 0.0).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.98768834059513777).abs() < 1.0e-3);
            assert!((cos - -0.15643446504023081).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.95105651629515364).abs() < 1.0e-3);
            assert!((cos - -0.30901699437494734).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.8910065241883679).abs() < 1.0e-3);
            assert!((cos - -0.45399049973954669).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.80901699437494745).abs() < 1.0e-3);
            assert!((cos - -0.58778525229247303).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.70710678118654757).abs() < 1.0e-3);
            assert!((cos - -0.70710678118654746).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.58778525229247325).abs() < 1.0e-3);
            assert!((cos - -0.80901699437494734).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.45399049973954686).abs() < 1.0e-3);
            assert!((cos - -0.89100652418836779).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.30901699437494751).abs() < 1.0e-3);
            assert!((cos - -0.95105651629515353).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.15643446504023098).abs() < 1.0e-3);
            assert!((cos - -0.98768834059513766).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 1.0e-3);
            assert!((cos - -1.0).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.15643446504023073).abs() < 1.0e-3);
            assert!((cos - -0.98768834059513777).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.30901699437494728).abs() < 1.0e-3);
            assert!((cos - -0.95105651629515364).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.45399049973954669).abs() < 1.0e-3);
            assert!((cos - -0.8910065241883679).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.58778525229247303).abs() < 1.0e-3);
            assert!((cos - -0.80901699437494756).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.70710678118654746).abs() < 1.0e-3);
            assert!((cos - -0.70710678118654768).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.80901699437494734).abs() < 1.0e-3);
            assert!((cos - -0.58778525229247325).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.89100652418836779).abs() < 1.0e-3);
            assert!((cos - -0.45399049973954692).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.95105651629515353).abs() < 1.0e-3);
            assert!((cos - -0.30901699437494756).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.98768834059513766).abs() < 1.0e-3);
            assert!((cos - -0.15643446504023104).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -1.0).abs() < 1.0e-3);
            assert!((cos - 0.0).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.98768834059513777).abs() < 1.0e-3);
            assert!((cos - 0.15643446504023067).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.95105651629515364).abs() < 1.0e-3);
            assert!((cos - 0.30901699437494723).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.8910065241883679).abs() < 1.0e-3);
            assert!((cos - 0.45399049973954664).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.80901699437494756).abs() < 1.0e-3);
            assert!((cos - 0.58778525229247292).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.70710678118654768).abs() < 1.0e-3);
            assert!((cos - 0.70710678118654735).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.58778525229247336).abs() < 1.0e-3);
            assert!((cos - 0.80901699437494734).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.45399049973954697).abs() < 1.0e-3);
            assert!((cos - 0.89100652418836779).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.30901699437494762).abs() < 1.0e-3);
            assert!((cos - 0.95105651629515353).abs() < 1.0e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.15643446504023109).abs() < 1.0e-3);
            assert!((cos - 0.98768834059513766).abs() < 1.0e-3);
        }
    }

    #[test]
    fn test_err_64() {
        // Tests double precision error accumulation over a relatively large number of
        // iterations.

        let mut o = QuadOsc::new(0.0, PI64 / 20.0);

        for _ in 0..8192 {
            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 1.0e-12);
            assert!((cos - 1.0).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.15643446504023087).abs() < 1.0e-12);
            assert!((cos - 0.98768834059513777).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.3090169943749474).abs() < 1.0e-12);
            assert!((cos - 0.95105651629515353).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.45399049973954675).abs() < 1.0e-12);
            assert!((cos - 0.8910065241883679).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.58778525229247314).abs() < 1.0e-12);
            assert!((cos - 0.80901699437494745).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.70710678118654746).abs() < 1.0e-12);
            assert!((cos - 0.70710678118654757).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.80901699437494745).abs() < 1.0e-12);
            assert!((cos - 0.58778525229247314).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.89100652418836779).abs() < 1.0e-12);
            assert!((cos - 0.4539904997395468).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.95105651629515353).abs() < 1.0e-12);
            assert!((cos - 0.30901699437494745).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.98768834059513777).abs() < 1.0e-12);
            assert!((cos - 0.15643446504023092).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 1.0).abs() < 1.0e-12);
            assert!((cos - 0.0).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.98768834059513777).abs() < 1.0e-12);
            assert!((cos - -0.15643446504023081).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.95105651629515364).abs() < 1.0e-12);
            assert!((cos - -0.30901699437494734).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.8910065241883679).abs() < 1.0e-12);
            assert!((cos - -0.45399049973954669).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.80901699437494745).abs() < 1.0e-12);
            assert!((cos - -0.58778525229247303).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.70710678118654757).abs() < 1.0e-12);
            assert!((cos - -0.70710678118654746).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.58778525229247325).abs() < 1.0e-12);
            assert!((cos - -0.80901699437494734).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.45399049973954686).abs() < 1.0e-12);
            assert!((cos - -0.89100652418836779).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.30901699437494751).abs() < 1.0e-12);
            assert!((cos - -0.95105651629515353).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.15643446504023098).abs() < 1.0e-12);
            assert!((cos - -0.98768834059513766).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 1.0e-12);
            assert!((cos - -1.0).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.15643446504023073).abs() < 1.0e-12);
            assert!((cos - -0.98768834059513777).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.30901699437494728).abs() < 1.0e-12);
            assert!((cos - -0.95105651629515364).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.45399049973954669).abs() < 1.0e-12);
            assert!((cos - -0.8910065241883679).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.58778525229247303).abs() < 1.0e-12);
            assert!((cos - -0.80901699437494756).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.70710678118654746).abs() < 1.0e-12);
            assert!((cos - -0.70710678118654768).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.80901699437494734).abs() < 1.0e-12);
            assert!((cos - -0.58778525229247325).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.89100652418836779).abs() < 1.0e-12);
            assert!((cos - -0.45399049973954692).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.95105651629515353).abs() < 1.0e-12);
            assert!((cos - -0.30901699437494756).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.98768834059513766).abs() < 1.0e-12);
            assert!((cos - -0.15643446504023104).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -1.0).abs() < 1.0e-12);
            assert!((cos - 0.0).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.98768834059513777).abs() < 1.0e-12);
            assert!((cos - 0.15643446504023067).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.95105651629515364).abs() < 1.0e-12);
            assert!((cos - 0.30901699437494723).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.8910065241883679).abs() < 1.0e-12);
            assert!((cos - 0.45399049973954664).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.80901699437494756).abs() < 1.0e-12);
            assert!((cos - 0.58778525229247292).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.70710678118654768).abs() < 1.0e-12);
            assert!((cos - 0.70710678118654735).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.58778525229247336).abs() < 1.0e-12);
            assert!((cos - 0.80901699437494734).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.45399049973954697).abs() < 1.0e-12);
            assert!((cos - 0.89100652418836779).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.30901699437494762).abs() < 1.0e-12);
            assert!((cos - 0.95105651629515353).abs() < 1.0e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.15643446504023109).abs() < 1.0e-12);
            assert!((cos - 0.98768834059513766).abs() < 1.0e-12);
        }
    }

    #[test]
    fn test_mult() {
        let m = 120;
        let common = 2.0 / 256 as f32 * PI32 * m as f32;
        let mut osc = QuadOsc::new(common * -104 as f32, common);

        for n in -104..105 {
            let inner = 2.0 / 256 as f32 * PI32 * m as f32 * n as f32;
            let (sin, cos) = inner.sin_cos();
            let (osin, ocos) = osc.next();

            assert!((sin - osin).abs() < 0.0001);
            assert!((cos - ocos).abs() < 0.0001);
        }
    }
}
