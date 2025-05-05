# 🦀 Rust Challenge – Integer to Alphanumeric Conversion

📎 Solved from this challenge post on substack:
https://weeklyrust.substack.com/i/162337676/rust-challenge

The challenge is to build a **Integer to Alphanumeric Conversion** in Rust that:

- Given a non-negative integer `num`, convert it to a base-36 string where:
  - Digits `0–9` are represented as `'0'–'9'`
  - Values `10–35` are represented as `'a'–'z'`
- The output should be in lowercase with no leading zeros.
- If the input is `0`, return `"0"`.

## 🛠️ Requirements

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)

## Examples

| Input      | Output   |
| ---------- | -------- |
| 46656      | "1000"   |
| 2147483646 | "zik0zi" |
| 2147483647 | "zik0zj" |
| 1295       | "zz"     |

## 🚀 How to Run

### 1. Clone the repository

```bash
git clone https://github.com/Silen1t/Rust-Bytes-Challenge-Issue-64-Solution-Silen1t.git
cd Rust-Bytes-Challenge-Issue-64-Solution-Silen1t
```

### 2. Build the project

```bash
cargo build --release
```

### 3. Run the application

```bash
cargo run --release
```
