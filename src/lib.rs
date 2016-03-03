/// Quadrature oscillator.
pub struct QuadOsc {
    /// sin and cos of phase step.
    step: (f32, f32),
    /// sin and cos of current phase.
    phase: (f32, f32),
}

impl QuadOsc {
    /// Phase step and initial phase.
    pub fn new(phase: f32, step: f32) -> QuadOsc {
        QuadOsc {
            step: step.sin_cos(),
            phase: phase.sin_cos(),
        }
    }

    pub fn set_step(&mut self, step: f32) {
        self.step = step.sin_cos();
    }

    /// Step the phase and return the current sin, cos pair.
    pub fn next(&mut self) -> (f32, f32) {
        let cur = self.phase;

        self.phase = (
            self.phase.0 * self.step.1 + self.phase.1 * self.step.0,
            self.phase.1 * self.step.1 - self.phase.0 * self.step.0,
        );

        cur
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_osc() {
        let mut o = QuadOsc::new(0.0, PI / 2.0);

        for _ in 0..100 {
            let (sin, cos) = o.next();
            println!("{} {}", sin, cos);
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
    fn test_mult() {
        let m = 120;
        let common = 2.0 / 256 as f32 * PI * m as f32;
        let mut osc = QuadOsc::new(common * -104 as f32, common);

        for n in -104..105 {
            let inner = 2.0 / 256 as f32 * PI * m as f32 * n as f32;
            let (sin, cos) = inner.sin_cos();
            let (osin, ocos) = osc.next();

            assert!((sin - osin).abs() < 0.0001);
            assert!((cos - ocos).abs() < 0.0001);
        }
    }
}
