//! Expression evaluation module

use anyhow::{bail, Result};
use meval::{Context, Expr};
use std::collections::HashMap;
use std::sync::Arc;

use super::math_functions::*;
use super::mods::{Mod, ModManager};
use super::random::*;

/// Evaluates mathematical expressions
pub struct Evaluator {
    /// Whether to use safe evaluation mode
    safe_mode: bool,

    /// Context with custom functions
    context: Context<'static>,

    /// Mod manager for custom mod functions
    mod_manager: ModManager,

    /// Expression cache for performance optimization
    expr_cache: HashMap<String, Arc<Expr>>,

    /// Result cache for performance optimization
    result_cache: HashMap<String, f64>,
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

        let mut mod_manager = ModManager::new();
        let _ = mod_manager.load_mods(); // Silently ignore errors if mods dir doesn't exist

        Self {
            safe_mode: true,
            context: ctx,
            mod_manager,
            expr_cache: HashMap::new(),
            result_cache: HashMap::new(),
        }
    }

    /// Sets the evaluation mode
    pub fn set_safe_mode(&mut self, safe: bool) {
        self.safe_mode = safe;
    }

    /// Reload all mods
    pub fn reload_mods(&mut self) -> Result<(), anyhow::Error> {
        self.mod_manager.reload_mods()
    }

    /// List all available mods
    pub fn list_mods(&self) -> Vec<String> {
        self.mod_manager.list_mods()
    }

    /// Get required variables for a mod
    pub fn get_required_vars(&self, name: &str) -> Option<Vec<String>> {
        self.mod_manager.get_required_vars(name)
    }

    /// Get warnings from mod loading
    pub fn get_warnings(&self) -> &[String] {
        self.mod_manager.get_warnings()
    }

    /// Clear warnings
    pub fn clear_warnings(&mut self) {
        self.mod_manager.clear_warnings();
    }

    /// Get a mod by name
    pub fn get_mod(&self, name: &str) -> Option<&Mod> {
        self.mod_manager.get_mod(name)
    }

    /// Evaluates a mathematical expression
    pub fn evaluate(&mut self, expression: &str) -> Result<f64> {
        // Check if expression is a mod function call (name(args))
        if let Some(paren_pos) = expression.find('(') {
            let func_name = expression[..paren_pos].trim();
            // We only need to check if the mod exists, we don't need to use the mod_def value here
            if self.mod_manager.get_mod(func_name).is_some() {
                // This is a mod function call
                return self.evaluate_mod(func_name, expression);
            }
        }

        // Otherwise, evaluate normally
        if self.safe_mode {
            // 使用缓存机制来优化性能
            let cache_key = expression.to_string();
            if let Some(cached_result) = self.get_cached_result(&cache_key) {
                return Ok(cached_result);
            }

            // 使用缓存的表达式对象来优化性能
            let expr = if let Some(cached_expr) = self.expr_cache.get(cache_key.as_str()) {
                cached_expr.clone()
            } else {
                let parsed_expr = expression.parse::<Expr>()?;
                let arc_expr = Arc::new(parsed_expr);
                self.expr_cache.insert(cache_key.clone(), arc_expr.clone());
                arc_expr
            };

            // 评估表达式
            match expr.eval_with_context(&self.context) {
                Ok(result) => {
                    // 缓存结果
                    self.result_cache.insert(cache_key, result);
                    Ok(result)
                }
                Err(e) => bail!("Evaluation error: {}", e),
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

    /// Evaluate a mod function call
    fn evaluate_mod(&self, mod_name: &str, expression: &str) -> Result<f64> {
        // Extract function name and arguments
        let paren_start = expression
            .find('(')
            .ok_or_else(|| anyhow::anyhow!("Invalid mod call"))?;
        let paren_end = expression
            .rfind(')')
            .ok_or_else(|| anyhow::anyhow!("Invalid mod call"))?;

        if paren_end <= paren_start {
            bail!("Invalid mod call: empty parentheses");
        }

        let args_str = &expression[paren_start + 1..paren_end];
        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

        // Get mod definition
        let mod_def = self
            .mod_manager
            .get_mod(mod_name)
            .ok_or_else(|| anyhow::anyhow!("Mod '{}' not found", mod_name))?;

        // Check number of arguments
        if args.len() != mod_def.var.needvars.len() {
            bail!(
                "Mod '{}' expects {} arguments, got {}",
                mod_name,
                mod_def.var.needvars.len(),
                args.len()
            );
        }

        // Build a new context with the provided arguments
        let mut ctx = self.context.clone();

        // Evaluate each argument and bind to variable
        for (i, var_name) in mod_def.var.needvars.iter().enumerate() {
            let arg_val: f64 = match args[i].parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    // Try to evaluate as an expression
                    match args[i].parse::<Expr>() {
                        Ok(expr) => match expr.eval_with_context(&self.context) {
                            Ok(v) => v,
                            Err(e) => bail!("Failed to evaluate argument '{}': {}", args[i], e),
                        },
                        Err(e) => bail!("Failed to parse argument '{}': {}", args[i], e),
                    }
                }
            };

            ctx.var(var_name, arg_val);
        }

        // Get the calculation expression
        let calc_expr = mod_def
            .calc
            .howto
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Mod '{}' has no calculation defined", mod_name))?;

        // Evaluate the calculation expression
        match calc_expr.parse::<Expr>() {
            Ok(expr) => match expr.eval_with_context(&ctx) {
                Ok(result) => Ok(result),
                Err(e) => bail!("Mod calculation error: {}", e),
            },
            Err(e) => bail!("Mod expression parse error: {}", e),
        }
    }

    // 添加缓存机制
    fn get_cached_result(&self, cache_key: &str) -> Option<f64> {
        // 实现缓存逻辑
        self.result_cache.get(cache_key).copied()
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
