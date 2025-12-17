use calculator_max::calculator::evaluator::Evaluator;
use calculator_max::calculator::math_functions::*;
use meval::Context;
use std::time::Instant;

fn bench_evaluator_parse_and_eval(ev: &mut Evaluator, expr: &str, iterations: usize) -> f64 {
    let mut sum = 0.0_f64;
    let start = Instant::now();
    for _ in 0..iterations {
        if let Ok(v) = ev.evaluate(expr) {
            sum += v;
        }
    }
    let elapsed = start.elapsed();
    println!("evaluator parse+eval '{}' x{} -> {:?}", expr, iterations, elapsed);
    sum
}

fn make_context() -> Context<'static> {
    let mut ctx = Context::new();
    ctx.var("pi", pi());
    ctx.var("e", e());
    ctx.func("sin", sin);
    ctx.func("cos", cos);
    ctx.func("tan", tan);
    ctx.func("asin", asin);
    ctx.func("acos", acos);
    ctx.func("atan", atan);
    ctx.func("sinh", sinh);
    ctx.func("cosh", cosh);
    ctx.func("tanh", tanh);
    ctx.func("exp", exp);
    ctx.func("sqrt", sqrt);
    ctx.func("log", log);
    ctx.func("log10", log10);
    ctx.func("log2", log2);
    ctx.func("ceil", ceil);
    ctx.func("floor", floor);
    ctx.func("trunc", trunc);
    ctx.func("fabs", fabs);
    ctx.func("factorial", factorial);
    ctx.func("gamma", gamma);
    ctx.func("erf", erf);
    ctx.func("erfc", erfc);
    ctx.func("degrees", degrees);
    ctx.func("radians", radians);
    ctx.func("s_circle", circle_area);
    ctx.funcn("s_tri", |args| triangle_area(args[0], args[1]), 2);
    ctx.funcn("s_rect", |args| rectangle_area(args[0], args[1]), 2);
    ctx
}

fn bench_parse_then_eval_with_context(expr: &str, iterations: usize, ctx: &Context) -> f64 {
    let mut sum = 0.0_f64;
    if let Ok(parsed) = expr.parse::<meval::Expr>() {
        let start = Instant::now();
        for _ in 0..iterations {
            if let Ok(v) = parsed.eval_with_context(ctx) {
                sum += v;
            }
        }
        let elapsed = start.elapsed();
        println!("parse-then-eval '{}' x{} -> {:?}", expr, iterations, elapsed);
    }
    sum
}

fn main() {
    let mut ev = Evaluator::new();
    let exprs = [
        "2+2*3",
        "sin(pi/4) + cos(pi/4)",
        "sqrt(12345.6789)",
        "factorial(10)",
        "s_rect(3,4)",
    ];

    let iterations = 200_00; // 200k
    let mut total = 0.0_f64;

    let ctx = make_context();

    for e in exprs.iter() {
        total += bench_evaluator_parse_and_eval(&mut ev, e, iterations);
        total += bench_parse_then_eval_with_context(e, iterations, &ctx);
    }    println!("total checksum={}", total);
}
