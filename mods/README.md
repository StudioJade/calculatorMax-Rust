# Mod Development Guide

This directory contains custom function mods for Calculator Max. Mods allow you to define your own functions using the `.cmfun` format.

## Getting Started

### File Format

Mod files use the `.cmfun` extension and are written in TOML format. Each mod file defines a single custom function.

### Basic Structure

```toml
[desc]
name = "function_name"

[var]
needvars = ["param1", "param2", ...]

[calc]
howto = "expression_using_parameters"
```

### File Components

#### `[desc]` Section
- **name**: The name of your custom function. This is how you'll call it in calculations.
  - Optional: If not provided, the filename (without extension) will be used as the function name.

#### `[var]` Section
- **needvars**: An array of parameter names that your function accepts.
  - Example: `["a", "b"]` means your function takes two parameters named `a` and `b`.
  - The order matters: they'll be matched to arguments in the order they're called.

#### `[calc]` Section
- **howto**: The calculation expression using the parameter names.
  - You can use all standard mathematical operations and functions.
  - Use the parameter names directly in the expression.

## Examples

### Example 1: Simple Addition

**File: `add.cmfun`**
```toml
[desc]
name = "add"

[var]
needvars = ["a", "b"]

[calc]
howto = "a + b"
```

**Usage:**
```
add(5, 3)      → 8
add(2.5, 1.5)  → 4
```

### Example 2: Area of Rectangle

**File: `rect_area.cmfun`**
```toml
[desc]
name = "rect_area"

[var]
needvars = ["width", "height"]

[calc]
howto = "width * height"
```

**Usage:**
```
rect_area(10, 5)   → 50
rect_area(3.5, 4)  → 14
```

### Example 3: Quadratic Formula (One Root)

**File: `quadratic.cmfun`**
```toml
[desc]
name = "quadratic"

[var]
needvars = ["a", "b", "c"]

[calc]
howto = "(-b + sqrt(b * b - 4 * a * c)) / (2 * a)"
```

**Usage:**
```
quadratic(1, -5, 6)  → 3
```

### Example 4: Convert Celsius to Fahrenheit

**File: `c_to_f.cmfun`**
```toml
[desc]
name = "celsius_to_fahrenheit"

[var]
needvars = ["celsius"]

[calc]
howto = "celsius * 9 / 5 + 32"
```

**Usage:**
```
celsius_to_fahrenheit(0)    → 32
celsius_to_fahrenheit(100)  → 212
```

## Supported Functions and Operations

You can use all standard mathematical functions and operations in your `howto` expression:

### Basic Operations
- Addition: `a + b`
- Subtraction: `a - b`
- Multiplication: `a * b`
- Division: `a / b`
- Exponentiation: `a ^ b` (not supported, use `exp` and `log` instead)

### Mathematical Functions
- `sin(x)`, `cos(x)`, `tan(x)` - Trigonometric functions
- `asin(x)`, `acos(x)`, `atan(x)` - Inverse trigonometric functions
- `sinh(x)`, `cosh(x)`, `tanh(x)` - Hyperbolic functions
- `sqrt(x)` - Square root
- `exp(x)` - Exponential function
- `log(x)` - Natural logarithm
- `log10(x)` - Base-10 logarithm
- `log2(x)` - Base-2 logarithm
- `ceil(x)` - Ceiling (round up)
- `floor(x)` - Floor (round down)
- `trunc(x)` - Truncate (remove decimal part)
- `fabs(x)` - Absolute value
- `factorial(x)` - Factorial (only for integers)

### Constants
- `pi` - π (3.14159...)
- `e` - Euler's number (2.71828...)

## Tips

1. **Parameter Names**: Use descriptive names for clarity. For example, use `radius` instead of `r`, or `height` instead of `h`.

2. **Order Matters**: The order of parameters in `needvars` must match the order you pass arguments when calling the function.

3. **Complex Expressions**: You can combine multiple operations and functions.
   ```toml
   howto = "a * sin(b) + c / d"
   ```

4. **Testing**: Test your mods by calling them in Calculator Max with various inputs to ensure they work correctly.

5. **Error Handling**: If there's an error in your expression (division by zero, invalid input, etc.), Calculator Max will display an error message.

## File Organization

Place your `.cmfun` files directly in the `mods/` directory. They will be automatically discovered and loaded when Calculator Max starts.

```
mods/
├── add.cmfun
├── multiply.cmfun
├── rect_area.cmfun
└── README.md (this file)
```

## Troubleshooting

- **Mod not loading**: Check that the file extension is exactly `.cmfun` and the filename contains no spaces.
- **Mod name conflicts**: If two mods have the same name, the last one loaded will be used. Ensure unique names.
- **Parse errors**: Double-check your TOML syntax. Make sure all array brackets are closed and strings are quoted.
- **Expression errors**: Verify that your expressions use correct syntax and available functions.

## Advanced Examples

### Pythagorean Theorem

```toml
[desc]
name = "pythagorean"

[var]
needvars = ["a", "b"]

[calc]
howto = "sqrt(a * a + b * b)"
```

Usage: `pythagorean(3, 4)` → 5

### Volume of Sphere

```toml
[desc]
name = "sphere_volume"

[var]
needvars = ["radius"]

[calc]
howto = "4 * pi * radius * radius * radius / 3"
```

Usage: `sphere_volume(5)` → 523.5987755982989

### Distance Formula

```toml
[desc]
name = "distance"

[var]
needvars = ["x1", "y1", "x2", "y2"]

[calc]
howto = "sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))"
```

Usage: `distance(0, 0, 3, 4)` → 5

---

Happy mod developing! If you create useful mods, feel free to share them with the community!
