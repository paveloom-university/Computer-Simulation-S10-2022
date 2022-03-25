//! Provides the [`runge_kutta_4th`] macro, plus tests for the method

/// Defines the [`runge_kutta_4th`](super::Integrators#method.runge_kutta_4th) method
#[macro_export]
macro_rules! runge_kutta_4th {
    () => {
        /// Integrate the system using the 4th-order Runge-Kutta method
        /// for `n` iterations with time step `h`, starting from time `t_0`
        ///
        /// Arguments:
        /// * `t_0` --- Initial value of time;
        /// * `h` --- Time step;
        /// * `n` --- Number of iterations;
        /// * `result` --- Result matrix;
        /// * `token` --- Private token.
        #[replace_float_literals(F::from(literal).unwrap())]
        fn runge_kutta_4th(&self, t_0: F, h: F, n: usize, result: &mut Result<F>, _: &Token) {
            // Integrate
            for i in 0..n {
                // Compute the time moments
                let t = t_0 + F::from(i).unwrap() * h;
                let t_2 = t + h / 2.;
                let t_3 = t_2;
                let t_4 = t + h;
                // Get the current state
                let x = result.state(i);
                // Compute the first increment
                let k_1 = &self.update(t, &x);
                // Compute the modified state for the second increment
                let x_m: Vec<F> = x
                    .iter()
                    .zip(k_1.iter())
                    .map(|(&x, &k_1)| x + h * k_1 / 2.)
                    .collect();
                // Compute the second increment
                let k_2 = self.update(t_2, &x_m);
                // Compute the modified state for the third increment
                let x_m: Vec<F> = x
                    .iter()
                    .zip(k_2.iter())
                    .map(|(&x, &k_2)| x + h * k_2 / 2.)
                    .collect();
                // Compute the third increment
                let k_3 = self.update(t_3, &x_m);
                // Compute the modified state for the fourth increment
                let x_m: Vec<F> = x
                    .iter()
                    .zip(k_3.iter())
                    .map(|(&x, &k_3)| x + h * k_3)
                    .collect();
                // Compute the fourth increment
                let k_4 = self.update(t_4, &x_m);
                // Compute the final modified state
                let x_m = x
                    .iter()
                    .zip(k_1.iter())
                    .zip(k_2.iter())
                    .zip(k_3.iter())
                    .zip(k_4.iter())
                    .map(|((((&x, &k_1), &k_2), &k_3), &k_4)| {
                        x + h / 6. * (k_1 + 2. * k_2 + 2. * k_3 + k_4)
                    })
                    .collect();
                // Put the new state in the result
                result.set_state(i + 1, x_m);
            }
        }
    };
}

#[cfg(test)]
use crate::test;

#[cfg(test)]
test!(runge_kutta_4th);
