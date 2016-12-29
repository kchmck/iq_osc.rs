# iq\_osc.rs

[Documentation](http://kchmck.github.io/doc/iq_osc/)

Given f(t) = cos(θ<sub>0</sub> + ωt) = cos Φ(t), an oscillator is defined here
to evaluate f(0), f(1), f(2), ... in sequence to generate a sinusoidal signal.
Further, a quadrature oscillator also evaluates g(t) = sin Φ(t) at each step
for the quadrature signal.

Computing these trig functions at each evaluation of f(t) and g(t) can be costly with
a high sample rate or within a tight loop. As an alternative, this crate implements a
quadrature oscillator that replaces the 2 trig function calls at each evaluation with
6 arithmetic operations (4 multiplies, 1 addition, and 1 subtraction.)
