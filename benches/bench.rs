#![feature(test)]

extern crate test;
extern crate iq_osc;

use std::f32::consts::PI as PI32;
use std::f64::consts::PI as PI64;
use iq_osc::IQOsc;

#[bench]
fn bench_trig32(b: &mut test::Bencher) {
    b.iter(|| {
        let mut k = 0;
        let freq = 2.0 * PI32 / 40.0;

        for _ in 0..256 {
            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.0).abs() < 100.0);
            assert!((cos - 1.0).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.15643446504023087).abs() < 100.0);
            assert!((cos - 0.98768834059513777).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.3090169943749474).abs() < 100.0);
            assert!((cos - 0.95105651629515353).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.45399049973954675).abs() < 100.0);
            assert!((cos - 0.8910065241883679).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.58778525229247314).abs() < 100.0);
            assert!((cos - 0.80901699437494745).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.70710678118654746).abs() < 100.0);
            assert!((cos - 0.70710678118654757).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.80901699437494745).abs() < 100.0);
            assert!((cos - 0.58778525229247314).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.89100652418836779).abs() < 100.0);
            assert!((cos - 0.4539904997395468).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.95105651629515353).abs() < 100.0);
            assert!((cos - 0.30901699437494745).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.98768834059513777).abs() < 100.0);
            assert!((cos - 0.15643446504023092).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 1.0).abs() < 100.0);
            assert!((cos - 0.0).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.98768834059513777).abs() < 100.0);
            assert!((cos - -0.15643446504023081).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.95105651629515364).abs() < 100.0);
            assert!((cos - -0.30901699437494734).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.8910065241883679).abs() < 100.0);
            assert!((cos - -0.45399049973954669).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.80901699437494745).abs() < 100.0);
            assert!((cos - -0.58778525229247303).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.70710678118654757).abs() < 100.0);
            assert!((cos - -0.70710678118654746).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.58778525229247325).abs() < 100.0);
            assert!((cos - -0.80901699437494734).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.45399049973954686).abs() < 100.0);
            assert!((cos - -0.89100652418836779).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.30901699437494751).abs() < 100.0);
            assert!((cos - -0.95105651629515353).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.15643446504023098).abs() < 100.0);
            assert!((cos - -0.98768834059513766).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - 0.0).abs() < 100.0);
            assert!((cos - -1.0).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.15643446504023073).abs() < 100.0);
            assert!((cos - -0.98768834059513777).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.30901699437494728).abs() < 100.0);
            assert!((cos - -0.95105651629515364).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.45399049973954669).abs() < 100.0);
            assert!((cos - -0.8910065241883679).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.58778525229247303).abs() < 100.0);
            assert!((cos - -0.80901699437494756).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.70710678118654746).abs() < 100.0);
            assert!((cos - -0.70710678118654768).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.80901699437494734).abs() < 100.0);
            assert!((cos - -0.58778525229247325).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.89100652418836779).abs() < 100.0);
            assert!((cos - -0.45399049973954692).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.95105651629515353).abs() < 100.0);
            assert!((cos - -0.30901699437494756).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.98768834059513766).abs() < 100.0);
            assert!((cos - -0.15643446504023104).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -1.0).abs() < 100.0);
            assert!((cos - 0.0).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.98768834059513777).abs() < 100.0);
            assert!((cos - 0.15643446504023067).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.95105651629515364).abs() < 100.0);
            assert!((cos - 0.30901699437494723).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.8910065241883679).abs() < 100.0);
            assert!((cos - 0.45399049973954664).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.80901699437494756).abs() < 100.0);
            assert!((cos - 0.58778525229247292).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.70710678118654768).abs() < 100.0);
            assert!((cos - 0.70710678118654735).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.58778525229247336).abs() < 100.0);
            assert!((cos - 0.80901699437494734).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.45399049973954697).abs() < 100.0);
            assert!((cos - 0.89100652418836779).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.30901699437494762).abs() < 100.0);
            assert!((cos - 0.95105651629515353).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f32).sin_cos();
            assert!((sin - -0.15643446504023109).abs() < 100.0);
            assert!((cos - 0.98768834059513766).abs() < 100.0);
            k += 1;
        }
    });
}

#[bench]
fn bench_trig64(b: &mut test::Bencher) {
    b.iter(|| {
        let mut k = 0;
        let freq = 2.0 * PI64 / 40.0;

        for _ in 0..256 {
            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.0).abs() < 100.0);
            assert!((cos - 1.0).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.15643446504023087).abs() < 100.0);
            assert!((cos - 0.98768834059513777).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.3090169943749474).abs() < 100.0);
            assert!((cos - 0.95105651629515353).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.45399049973954675).abs() < 100.0);
            assert!((cos - 0.8910065241883679).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.58778525229247314).abs() < 100.0);
            assert!((cos - 0.80901699437494745).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.70710678118654746).abs() < 100.0);
            assert!((cos - 0.70710678118654757).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.80901699437494745).abs() < 100.0);
            assert!((cos - 0.58778525229247314).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.89100652418836779).abs() < 100.0);
            assert!((cos - 0.4539904997395468).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.95105651629515353).abs() < 100.0);
            assert!((cos - 0.30901699437494745).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.98768834059513777).abs() < 100.0);
            assert!((cos - 0.15643446504023092).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 1.0).abs() < 100.0);
            assert!((cos - 0.0).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.98768834059513777).abs() < 100.0);
            assert!((cos - -0.15643446504023081).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.95105651629515364).abs() < 100.0);
            assert!((cos - -0.30901699437494734).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.8910065241883679).abs() < 100.0);
            assert!((cos - -0.45399049973954669).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.80901699437494745).abs() < 100.0);
            assert!((cos - -0.58778525229247303).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.70710678118654757).abs() < 100.0);
            assert!((cos - -0.70710678118654746).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.58778525229247325).abs() < 100.0);
            assert!((cos - -0.80901699437494734).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.45399049973954686).abs() < 100.0);
            assert!((cos - -0.89100652418836779).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.30901699437494751).abs() < 100.0);
            assert!((cos - -0.95105651629515353).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.15643446504023098).abs() < 100.0);
            assert!((cos - -0.98768834059513766).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - 0.0).abs() < 100.0);
            assert!((cos - -1.0).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.15643446504023073).abs() < 100.0);
            assert!((cos - -0.98768834059513777).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.30901699437494728).abs() < 100.0);
            assert!((cos - -0.95105651629515364).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.45399049973954669).abs() < 100.0);
            assert!((cos - -0.8910065241883679).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.58778525229247303).abs() < 100.0);
            assert!((cos - -0.80901699437494756).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.70710678118654746).abs() < 100.0);
            assert!((cos - -0.70710678118654768).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.80901699437494734).abs() < 100.0);
            assert!((cos - -0.58778525229247325).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.89100652418836779).abs() < 100.0);
            assert!((cos - -0.45399049973954692).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.95105651629515353).abs() < 100.0);
            assert!((cos - -0.30901699437494756).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.98768834059513766).abs() < 100.0);
            assert!((cos - -0.15643446504023104).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -1.0).abs() < 100.0);
            assert!((cos - 0.0).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.98768834059513777).abs() < 100.0);
            assert!((cos - 0.15643446504023067).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.95105651629515364).abs() < 100.0);
            assert!((cos - 0.30901699437494723).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.8910065241883679).abs() < 100.0);
            assert!((cos - 0.45399049973954664).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.80901699437494756).abs() < 100.0);
            assert!((cos - 0.58778525229247292).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.70710678118654768).abs() < 100.0);
            assert!((cos - 0.70710678118654735).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.58778525229247336).abs() < 100.0);
            assert!((cos - 0.80901699437494734).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.45399049973954697).abs() < 100.0);
            assert!((cos - 0.89100652418836779).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.30901699437494762).abs() < 100.0);
            assert!((cos - 0.95105651629515353).abs() < 100.0);
            k += 1;

            let (sin, cos) = (freq * k as f64).sin_cos();
            assert!((sin - -0.15643446504023109).abs() < 100.0);
            assert!((cos - 0.98768834059513766).abs() < 100.0);
            k += 1;
        }
    });
}

#[bench]
fn bench_osc32(b: &mut test::Bencher) {
    b.iter(|| {
        let mut o = IQOsc::new(0.0, PI32 / 20.0);

        for _ in 0..256 {
            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 1e-3);
            assert!((cos - 1.0).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.15643446504023087).abs() < 1e-3);
            assert!((cos - 0.98768834059513777).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.3090169943749474).abs() < 1e-3);
            assert!((cos - 0.95105651629515353).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.45399049973954675).abs() < 1e-3);
            assert!((cos - 0.8910065241883679).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.58778525229247314).abs() < 1e-3);
            assert!((cos - 0.80901699437494745).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.70710678118654746).abs() < 1e-3);
            assert!((cos - 0.70710678118654757).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.80901699437494745).abs() < 1e-3);
            assert!((cos - 0.58778525229247314).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.89100652418836779).abs() < 1e-3);
            assert!((cos - 0.4539904997395468).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.95105651629515353).abs() < 1e-3);
            assert!((cos - 0.30901699437494745).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.98768834059513777).abs() < 1e-3);
            assert!((cos - 0.15643446504023092).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 1.0).abs() < 1e-3);
            assert!((cos - 0.0).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.98768834059513777).abs() < 1e-3);
            assert!((cos - -0.15643446504023081).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.95105651629515364).abs() < 1e-3);
            assert!((cos - -0.30901699437494734).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.8910065241883679).abs() < 1e-3);
            assert!((cos - -0.45399049973954669).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.80901699437494745).abs() < 1e-3);
            assert!((cos - -0.58778525229247303).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.70710678118654757).abs() < 1e-3);
            assert!((cos - -0.70710678118654746).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.58778525229247325).abs() < 1e-3);
            assert!((cos - -0.80901699437494734).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.45399049973954686).abs() < 1e-3);
            assert!((cos - -0.89100652418836779).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.30901699437494751).abs() < 1e-3);
            assert!((cos - -0.95105651629515353).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.15643446504023098).abs() < 1e-3);
            assert!((cos - -0.98768834059513766).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 1e-3);
            assert!((cos - -1.0).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.15643446504023073).abs() < 1e-3);
            assert!((cos - -0.98768834059513777).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.30901699437494728).abs() < 1e-3);
            assert!((cos - -0.95105651629515364).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.45399049973954669).abs() < 1e-3);
            assert!((cos - -0.8910065241883679).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.58778525229247303).abs() < 1e-3);
            assert!((cos - -0.80901699437494756).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.70710678118654746).abs() < 1e-3);
            assert!((cos - -0.70710678118654768).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.80901699437494734).abs() < 1e-3);
            assert!((cos - -0.58778525229247325).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.89100652418836779).abs() < 1e-3);
            assert!((cos - -0.45399049973954692).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.95105651629515353).abs() < 1e-3);
            assert!((cos - -0.30901699437494756).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.98768834059513766).abs() < 1e-3);
            assert!((cos - -0.15643446504023104).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -1.0).abs() < 1e-3);
            assert!((cos - 0.0).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.98768834059513777).abs() < 1e-3);
            assert!((cos - 0.15643446504023067).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.95105651629515364).abs() < 1e-3);
            assert!((cos - 0.30901699437494723).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.8910065241883679).abs() < 1e-3);
            assert!((cos - 0.45399049973954664).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.80901699437494756).abs() < 1e-3);
            assert!((cos - 0.58778525229247292).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.70710678118654768).abs() < 1e-3);
            assert!((cos - 0.70710678118654735).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.58778525229247336).abs() < 1e-3);
            assert!((cos - 0.80901699437494734).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.45399049973954697).abs() < 1e-3);
            assert!((cos - 0.89100652418836779).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.30901699437494762).abs() < 1e-3);
            assert!((cos - 0.95105651629515353).abs() < 1e-3);

            let (sin, cos) = o.next();
            assert!((sin - -0.15643446504023109).abs() < 1e-3);
            assert!((cos - 0.98768834059513766).abs() < 1e-3);
        }
    });
}

#[bench]
fn bench_osc64(b: &mut test::Bencher) {
    b.iter(|| {
        let mut o = IQOsc::new(0.0, PI64 / 20.0);

        for _ in 0..256 {
            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 1e-12);
            assert!((cos - 1.0).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.15643446504023087).abs() < 1e-12);
            assert!((cos - 0.98768834059513777).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.3090169943749474).abs() < 1e-12);
            assert!((cos - 0.95105651629515353).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.45399049973954675).abs() < 1e-12);
            assert!((cos - 0.8910065241883679).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.58778525229247314).abs() < 1e-12);
            assert!((cos - 0.80901699437494745).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.70710678118654746).abs() < 1e-12);
            assert!((cos - 0.70710678118654757).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.80901699437494745).abs() < 1e-12);
            assert!((cos - 0.58778525229247314).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.89100652418836779).abs() < 1e-12);
            assert!((cos - 0.4539904997395468).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.95105651629515353).abs() < 1e-12);
            assert!((cos - 0.30901699437494745).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.98768834059513777).abs() < 1e-12);
            assert!((cos - 0.15643446504023092).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 1.0).abs() < 1e-12);
            assert!((cos - 0.0).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.98768834059513777).abs() < 1e-12);
            assert!((cos - -0.15643446504023081).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.95105651629515364).abs() < 1e-12);
            assert!((cos - -0.30901699437494734).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.8910065241883679).abs() < 1e-12);
            assert!((cos - -0.45399049973954669).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.80901699437494745).abs() < 1e-12);
            assert!((cos - -0.58778525229247303).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.70710678118654757).abs() < 1e-12);
            assert!((cos - -0.70710678118654746).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.58778525229247325).abs() < 1e-12);
            assert!((cos - -0.80901699437494734).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.45399049973954686).abs() < 1e-12);
            assert!((cos - -0.89100652418836779).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.30901699437494751).abs() < 1e-12);
            assert!((cos - -0.95105651629515353).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.15643446504023098).abs() < 1e-12);
            assert!((cos - -0.98768834059513766).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - 0.0).abs() < 1e-12);
            assert!((cos - -1.0).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.15643446504023073).abs() < 1e-12);
            assert!((cos - -0.98768834059513777).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.30901699437494728).abs() < 1e-12);
            assert!((cos - -0.95105651629515364).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.45399049973954669).abs() < 1e-12);
            assert!((cos - -0.8910065241883679).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.58778525229247303).abs() < 1e-12);
            assert!((cos - -0.80901699437494756).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.70710678118654746).abs() < 1e-12);
            assert!((cos - -0.70710678118654768).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.80901699437494734).abs() < 1e-12);
            assert!((cos - -0.58778525229247325).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.89100652418836779).abs() < 1e-12);
            assert!((cos - -0.45399049973954692).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.95105651629515353).abs() < 1e-12);
            assert!((cos - -0.30901699437494756).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.98768834059513766).abs() < 1e-12);
            assert!((cos - -0.15643446504023104).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -1.0).abs() < 1e-12);
            assert!((cos - 0.0).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.98768834059513777).abs() < 1e-12);
            assert!((cos - 0.15643446504023067).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.95105651629515364).abs() < 1e-12);
            assert!((cos - 0.30901699437494723).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.8910065241883679).abs() < 1e-12);
            assert!((cos - 0.45399049973954664).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.80901699437494756).abs() < 1e-12);
            assert!((cos - 0.58778525229247292).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.70710678118654768).abs() < 1e-12);
            assert!((cos - 0.70710678118654735).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.58778525229247336).abs() < 1e-12);
            assert!((cos - 0.80901699437494734).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.45399049973954697).abs() < 1e-12);
            assert!((cos - 0.89100652418836779).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.30901699437494762).abs() < 1e-12);
            assert!((cos - 0.95105651629515353).abs() < 1e-12);

            let (sin, cos) = o.next();
            assert!((sin - -0.15643446504023109).abs() < 1e-12);
            assert!((cos - 0.98768834059513766).abs() < 1e-12);
        }
    });
}
