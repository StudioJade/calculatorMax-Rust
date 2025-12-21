//! Mathematical functions implementation

use std::f64::consts::{E, PI};

/// Calculates the area of a triangle
#[inline]
pub fn triangle_area(base: f64, height: f64) -> f64 {
    base * height * 0.5
}

/// Calculates the area of a rectangle
#[inline]
pub fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

/// Calculates the area of a circle
#[inline]
pub fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

/// Calculates the area of a trapezoid
#[inline]
pub fn trapezoid_area(base1: f64, base2: f64, height: f64) -> f64 {
    (base1 + base2) * height * 0.5
}

/// Calculates the area of a triangle using Heron's formula
#[inline]
pub fn heron_triangle_area(a: f64, b: f64, c: f64) -> Option<f64> {
    let s = (a + b + c) * 0.5;
    let area_squared = s * (s - a) * (s - b) * (s - c);

    if area_squared >= 0.0 {
        Some(area_squared.sqrt())
    } else {
        None // Invalid triangle
    }
}

/// Calculates the hypotenuse of a right triangle using Pythagorean theorem
#[inline]
pub fn pythagorean_theorem(a: f64, b: f64) -> f64 {
    (a * a + b * b).sqrt()
}

/// Provides access to mathematical constants
#[inline]
pub fn pi() -> f64 {
    PI
}

#[inline]
pub fn e() -> f64 {
    E
}

// Trigonometric functions
pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn cos(x: f64) -> f64 {
    x.cos()
}

pub fn csin(x: f64) -> f64 {
    x.sin().cos()
}

pub fn tan(x: f64) -> f64 {
    x.tan()
}

pub fn asin(x: f64) -> f64 {
    x.asin()
}

pub fn acos(x: f64) -> f64 {
    x.acos()
}

pub fn atan(x: f64) -> f64 {
    x.atan()
}

pub fn atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

// Hyperbolic functions
pub fn sinh(x: f64) -> f64 {
    x.sinh()
}

pub fn cosh(x: f64) -> f64 {
    x.cosh()
}

pub fn tanh(x: f64) -> f64 {
    x.tanh()
}

// Exponential and logarithmic functions
pub fn exp(x: f64) -> f64 {
    x.exp()
}

pub fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

pub fn log(x: f64) -> f64 {
    x.ln()
}

pub fn log10(x: f64) -> f64 {
    x.log10()
}

pub fn log2(x: f64) -> f64 {
    x.log2()
}

// Rounding functions
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

pub fn floor(x: f64) -> f64 {
    x.floor()
}

pub fn trunc(x: f64) -> f64 {
    x.trunc()
}

// Absolute value
pub fn fabs(x: f64) -> f64 {
    x.abs()
}

// Factorial (using gamma function)
pub fn factorial(x: f64) -> f64 {
    if x < 0.0 {
        f64::NAN
    } else if x.fract() == 0.0 && x <= 170.0 {
        // For integer values up to 170, calculate actual factorial
        let n = x as u64;
        let mut result = 1.0;
        for i in 1..=n {
            result *= i as f64;
        }
        result
    } else {
        // For non-integer or large values, use gamma function
        gamma(x + 1.0)
    }
}

// Gamma function (simplified implementation)
pub fn gamma(x: f64) -> f64 {
    statrs::function::gamma::gamma(x)
}

// Error functions
pub fn erf(x: f64) -> f64 {
    statrs::function::erf::erf(x)
}

pub fn erfc(x: f64) -> f64 {
    statrs::function::erf::erfc(x)
}

// GCD and LCM
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

// Angle conversion
pub fn degrees(x: f64) -> f64 {
    x * 180.0 / PI
}

pub fn radians(x: f64) -> f64 {
    x * PI / 180.0
}

// Floating point utilities
pub fn is_inf(x: f64) -> bool {
    x.is_infinite()
}

pub fn is_nan(x: f64) -> bool {
    x.is_nan()
}

pub fn is_close(a: f64, b: f64, rel_tol: f64, abs_tol: f64) -> bool {
    (a - b).abs() <= (rel_tol * b.abs()).max(abs_tol)
}
