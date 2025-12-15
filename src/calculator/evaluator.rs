//! Expression evaluation module

use anyhow::{bail, Result};
use meval::{Context, Expr};

use super::math_functions::*;
use super::random::*;

/// Evaluates mathematical expressions
pub struct Evaluator {
    /// Whether to use safe evaluation mode
    safe_mode: bool,

    /// Context with custom functions
    context: Context<'static>,
}

impl Evaluator {
    /// Creates a new evaluator
    pub fn new() -> Self {
        let mut ctx = Context::new();

        // Add mathematical constants
        ctx.var("pi", pi());
        ctx.var("e", e());

        // Add trigonometric functions
        ctx.func("sin", sin);
        ctx.func("cos", cos);
        ctx.func("tan", tan);
        ctx.func("asin", asin);
        ctx.func("acos", acos);
        ctx.func("atan", atan);

        // Add hyperbolic functions
        ctx.func("sinh", sinh);
        ctx.func("cosh", cosh);
        ctx.func("tanh", tanh);

        // Add exponential and logarithmic functions
        ctx.func("exp", exp);
        ctx.func("sqrt", sqrt);
        ctx.func("log", log);
        ctx.func("log10", log10);
        ctx.func("log2", log2);

        // Add rounding functions
        ctx.func("ceil", ceil);
        ctx.func("floor", floor);
        ctx.func("trunc", trunc);

        // Add absolute value
        ctx.func("fabs", fabs);

        // Add factorial and gamma functions
        ctx.func("factorial", factorial);
        ctx.func("gamma", gamma);

        // Add error functions
        ctx.func("erf", erf);
        ctx.func("erfc", erfc);

        // Add angle conversion functions
        ctx.func("degrees", degrees);
        ctx.func("radians", radians);

        // Add geometric functions (single argument versions)
        ctx.func("s_circle", circle_area);

        // Add geometric functions (two argument versions)
        ctx.funcn("s_tri", |args| triangle_area(args[0], args[1]), 2);
        ctx.funcn("s_rect", |args| rectangle_area(args[0], args[1]), 2);

        // Add random functions
        ctx.func("random", |_| random()); // Takes dummy parameter

        Self {
            safe_mode: true,
            context: ctx,
        }
    }

    /// Sets the evaluation mode
    pub fn set_safe_mode(&mut self, safe: bool) {
        self.safe_mode = safe;
    }

    /// Evaluates a mathematical expression
    pub fn evaluate(&self, expression: &str) -> Result<f64> {
        if self.safe_mode {
            // Safe evaluation using meval crate with custom context
            match expression.parse::<Expr>() {
                Ok(expr) => match expr.eval_with_context(&self.context) {
                    Ok(result) => Ok(result),
                    Err(e) => bail!("Evaluation error: {}", e),
                },
                Err(e) => bail!("Parse error: {}", e),
            }
        } else {
            // In a real implementation, this would allow more complex expressions
            // For now, we'll just use the same safe evaluation
            match expression.parse::<Expr>() {
                Ok(expr) => match expr.eval_with_context(&self.context) {
                    Ok(result) => Ok(result),
                    Err(e) => bail!("Evaluation error: {}", e),
                },
                Err(e) => bail!("Parse error: {}", e),
            }
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
