//! Integration tests for the calculator

#[cfg(test)]
mod tests {
    use calculator_max::calculator::{math_functions, Evaluator};

    #[test]
    fn test_basic_arithmetic() {
        let evaluator = Evaluator::new();

        assert_eq!(evaluator.evaluate("2 + 3").unwrap(), 5.0);
        assert_eq!(evaluator.evaluate("10 - 4").unwrap(), 6.0);
        assert_eq!(evaluator.evaluate("3 * 4").unwrap(), 12.0);
        assert_eq!(evaluator.evaluate("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_math_functions() {
        assert_eq!(math_functions::triangle_area(10.0, 5.0), 25.0);
        assert_eq!(math_functions::rectangle_area(4.0, 6.0), 24.0);
        assert_eq!(math_functions::circle_area(1.0), std::f64::consts::PI);
    }

    #[test]
    fn test_pythagorean_theorem() {
        let result = math_functions::pythagorean_theorem(3.0, 4.0);
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_math_constants() {
        let evaluator = Evaluator::new();

        assert!((evaluator.evaluate("pi").unwrap() - std::f64::consts::PI).abs() < 1e-10);
        assert!((evaluator.evaluate("e").unwrap() - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_trigonometric_functions() {
        let evaluator = Evaluator::new();

        assert!((evaluator.evaluate("sin(0)").unwrap() - 0.0).abs() < 1e-10);
        assert!((evaluator.evaluate("cos(0)").unwrap() - 1.0).abs() < 1e-10);
        assert!((evaluator.evaluate("sqrt(4)").unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_geometric_functions() {
        let evaluator = Evaluator::new();

        assert_eq!(evaluator.evaluate("s_tri(10, 5)").unwrap(), 25.0);
        assert_eq!(evaluator.evaluate("s_rect(4, 6)").unwrap(), 24.0);
    }
}
