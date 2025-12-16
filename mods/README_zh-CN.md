# Mod 开发指南

此目录包含计算器 Max 的自定义函数 mod。Mod 允许您使用 `.cmfun` 格式定义自己的函数。

## 快速开始

### 文件格式

Mod 文件使用 `.cmfun` 扩展名，采用 TOML 格式编写。每个 mod 文件定义一个自定义函数。

### 基本结构

```toml
[desc]
name = "函数名"

[var]
needvars = ["参数1", "参数2", ...]

[calc]
howto = "使用参数的计算表达式"
```

### 文件组件

#### `[desc]` 部分
- **name**: 自定义函数的名称。这是在计算中调用它的方式。
  - 可选：如果未提供，将使用文件名（不含扩展名）作为函数名。

#### `[var]` 部分
- **needvars**: 函数接受的参数名称数组。
  - 示例：`["a", "b"]` 表示函数接受两个名为 `a` 和 `b` 的参数。
  - 顺序很重要：它们将按调用时的顺序与参数匹配。

#### `[calc]` 部分
- **howto**: 使用参数名的计算表达式。
  - 您可以使用所有标准数学运算和函数。
  - 在表达式中直接使用参数名。

## 示例

### 示例 1: 简单加法

**文件: `add.cmfun`**
```toml
[desc]
name = "add"

[var]
needvars = ["a", "b"]

[calc]
howto = "a + b"
```

**使用方法:**
```
add(5, 3)      → 8
add(2.5, 1.5)  → 4
```

### 示例 2: 矩形面积

**文件: `rect_area.cmfun`**
```toml
[desc]
name = "rect_area"

[var]
needvars = ["width", "height"]

[calc]
howto = "width * height"
```

**使用方法:**
```
rect_area(10, 5)   → 50
rect_area(3.5, 4)  → 14
```

### 示例 3: 二次方程求根

**文件: `quadratic.cmfun`**
```toml
[desc]
name = "quadratic"

[var]
needvars = ["a", "b", "c"]

[calc]
howto = "(-b + sqrt(b * b - 4 * a * c)) / (2 * a)"
```

**使用方法:**
```
quadratic(1, -5, 6)  → 3
```

### 示例 4: 摄氏度转华氏度

**文件: `c_to_f.cmfun`**
```toml
[desc]
name = "celsius_to_fahrenheit"

[var]
needvars = ["celsius"]

[calc]
howto = "celsius * 9 / 5 + 32"
```

**使用方法:**
```
celsius_to_fahrenheit(0)    → 32
celsius_to_fahrenheit(100)  → 212
```

## 支持的函数和运算

您可以在 `howto` 表达式中使用所有标准数学函数和运算：

### 基本运算
- 加法: `a + b`
- 减法: `a - b`
- 乘法: `a * b`
- 除法: `a / b`
- 幂运算: `a ^ b`（不支持）

### 数学函数
- `sin(x)`, `cos(x)`, `tan(x)` - 三角函数
- `asin(x)`, `acos(x)`, `atan(x)` - 反三角函数
- `sinh(x)`, `cosh(x)`, `tanh(x)` - 双曲函数
- `sqrt(x)` - 平方根
- `exp(x)` - 指数函数
- `log(x)` - 自然对数
- `log10(x)` - 以 10 为底的对数
- `log2(x)` - 以 2 为底的对数
- `ceil(x)` - 向上取整
- `floor(x)` - 向下取整
- `trunc(x)` - 截断（移除小数部分）
- `fabs(x)` - 绝对值
- `factorial(x)` - 阶乘（仅限整数）

### 常数
- `pi` - π (3.14159...)
- `e` - 欧拉数 (2.71828...)

## 提示

1. **参数名称**: 使用描述性名称以提高清晰度。例如，使用 `radius` 而不是 `r`，或使用 `height` 而不是 `h`。

2. **顺序很重要**: `needvars` 中参数的顺序必须与调用函数时传递的参数顺序匹配。

3. **复杂表达式**: 您可以组合多个运算和函数。
   ```toml
   howto = "a * sin(b) + c / d"
   ```

4. **测试**: 在计算器 Max 中调用您的 mod，使用各种输入来测试它们是否正常工作。

5. **错误处理**: 如果表达式中出现错误（除以零、无效输入等），计算器 Max 将显示错误消息。

## 文件组织

将您的 `.cmfun` 文件直接放在 `mods/` 目录中。当计算器 Max 启动时，它们将被自动发现并加载。

```
mods/
├── add.cmfun
├── multiply.cmfun
├── rect_area.cmfun
└── README.md (此文件)
```

## 故障排除

- **Mod 未加载**: 检查文件扩展名是否完全为 `.cmfun`，文件名中是否不包含空格。
- **Mod 名称冲突**: 如果两个 mod 具有相同的名称，最后加载的将被使用。确保名称唯一。
- **解析错误**: 重新检查 TOML 语法。确保所有数组括号都已关闭，字符串都已引用。
- **表达式错误**: 验证您的表达式使用正确的语法和可用函数。

## 高级示例

### 勾股定理

```toml
[desc]
name = "pythagorean"

[var]
needvars = ["a", "b"]

[calc]
howto = "sqrt(a * a + b * b)"
```

使用方法: `pythagorean(3, 4)` → 5

### 球体体积

```toml
[desc]
name = "sphere_volume"

[var]
needvars = ["radius"]

[calc]
howto = "4 * pi * radius * radius * radius / 3"
```

使用方法: `sphere_volume(5)` → 523.5987755982989

### 距离公式

```toml
[desc]
name = "distance"

[var]
needvars = ["x1", "y1", "x2", "y2"]

[calc]
howto = "sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))"
```

使用方法: `distance(0, 0, 3, 4)` → 5

---

祝您 mod 开发愉快！如果您创建了有用的 mod，欢迎与社区分享！
