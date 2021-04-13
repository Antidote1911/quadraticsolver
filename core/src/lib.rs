fn delta(a: &f64, b: &f64, c: &f64) -> f64 {
    b.powi(2) - (4.0 * a * c)
}

pub fn calculate_root(a: &f64, b: &f64, c: &f64) -> (f64, f64, f64, bool) {
    let mut are_real = true;


    // calculate the determinant
    let delta = delta(&a, &b, &c);

    // if delta < 0, solutions or not real
    if delta < 0.0 { are_real = false; }

    let (x1, x2): (f64, f64);
    x1 = ((-1.0 * b) + (delta as f64).sqrt() as f64) / (2.0 * a);
    x2 = ((-1.0 * b) - (delta as f64).sqrt() as f64) / (2.0 * a);

    (delta, x1, x2, are_real)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant() {
        assert_eq!(delta(
            &(10 as f64),
            &(10 as f64),
            &(-60 as f64),
        ), 2500 as f64);
    }

    #[test]
    fn solve() {
        let roots = calculate_root(
            &(10 as f64),
            &(10 as f64),
            &(-60 as f64),
        );
        assert_eq!(roots.0, 2500 as f64);
        assert_eq!(roots.1, 2 as f64);
        assert_eq!(roots.2, -3 as f64);
    }
}
