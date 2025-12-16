# Mod 開發指南

此目錄包含計算機 Max 的自訂函式 mod。Mod 允許您使用 `.cmfun` 格式定義自己的函式。

## 快速開始

### 檔案格式

Mod 檔案使用 `.cmfun` 副檔名，採用 TOML 格式編寫。每個 mod 檔案定義一個自訂函式。

### 基本結構

```toml
[desc]
name = "函式名"

[var]
needvars = ["參數1", "參數2", ...]

[calc]
howto = "使用參數的計算運算式"
```

### 檔案組件

#### `[desc]` 部分
- **name**: 自訂函式的名稱。這是在計算中呼叫它的方式。
  - 可選：如果未提供，將使用檔案名（不含副檔名）作為函式名。

#### `[var]` 部分
- **needvars**: 函式接受的參數名稱陣列。
  - 示例：`["a", "b"]` 表示函式接受兩個名為 `a` 和 `b` 的參數。
  - 順序很重要：它們將按呼叫時的順序與參數配對。

#### `[calc]` 部分
- **howto**: 使用參數名的計算運算式。
  - 您可以使用所有標準數學運算和函式。
  - 在運算式中直接使用參數名。

## 示例

### 示例 1: 簡單加法

**檔案: `add.cmfun`**
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

### 示例 2: 矩形面積

**檔案: `rect_area.cmfun`**
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

**檔案: `quadratic.cmfun`**
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

### 示例 4: 攝氏度轉華氏度

**檔案: `c_to_f.cmfun`**
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

## 支援的函式和運算

您可以在 `howto` 運算式中使用所有標準數學函式和運算：

### 基本運算
- 加法: `a + b`
- 減法: `a - b`
- 乘法: `a * b`
- 除法: `a / b`
- 冪運算: `a ^ b`（不支援）

### 數學函式
- `sin(x)`, `cos(x)`, `tan(x)` - 三角函式
- `asin(x)`, `acos(x)`, `atan(x)` - 反三角函式
- `sinh(x)`, `cosh(x)`, `tanh(x)` - 雙曲函式
- `sqrt(x)` - 平方根
- `exp(x)` - 指數函式
- `log(x)` - 自然對數
- `log10(x)` - 以 10 為底的對數
- `log2(x)` - 以 2 為底的對數
- `ceil(x)` - 向上取整
- `floor(x)` - 向下取整
- `trunc(x)` - 截斷（移除小數部分）
- `fabs(x)` - 絕對值
- `factorial(x)` - 階乘（僅限整數）

### 常數
- `pi` - π (3.14159...)
- `e` - 歐拉數 (2.71828...)

## 提示

1. **參數名稱**: 使用描述性名稱以提高清晰度。例如，使用 `radius` 而不是 `r`，或使用 `height` 而不是 `h`。

2. **順序很重要**: `needvars` 中參數的順序必須與呼叫函式時傳遞的參數順序相符。

3. **複雜運算式**: 您可以組合多個運算和函式。
   ```toml
   howto = "a * sin(b) + c / d"
   ```

4. **測試**: 在計算機 Max 中呼叫您的 mod，使用各種輸入來測試它們是否正常運作。

5. **錯誤處理**: 如果運算式中出現錯誤（除以零、無效輸入等），計算機 Max 將顯示錯誤訊息。

## 檔案組織

將您的 `.cmfun` 檔案直接放在 `mods/` 目錄中。當計算機 Max 啟動時，它們將被自動發現並載入。

```
mods/
├── add.cmfun
├── multiply.cmfun
├── rect_area.cmfun
└── README.md (此檔案)
```

## 故障排除

- **Mod 未載入**: 檢查檔案副檔名是否完全為 `.cmfun`，檔案名中是否不包含空格。
- **Mod 名稱衝突**: 如果兩個 mod 具有相同的名稱，最後載入的將被使用。確保名稱唯一。
- **解析錯誤**: 重新檢查 TOML 語法。確保所有陣列括號都已關閉，字串都已引用。
- **運算式錯誤**: 驗證您的運算式使用正確的語法和可用函式。

## 高級示例

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

### 球體體積

```toml
[desc]
name = "sphere_volume"

[var]
needvars = ["radius"]

[calc]
howto = "4 * pi * radius * radius * radius / 3"
```

使用方法: `sphere_volume(5)` → 523.5987755982989

### 距離公式

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

祝您 mod 開發愉快！如果您建立了有用的 mod，歡迎與社群分享！
