# YAD File Format

**YAD** is a lightweight binary file format designed for efficient data storage and inspired by the simplicity of **JSON** and the structure of **BSON**.  
Its goal is to provide a compact, tokenized, and easy-to-parse format while keeping the implementation minimal and human-readable through hex dumps.

---

## ✨ Key Benefits

- **Lightweight and compact**  
  Stores data in binary form, making it significantly smaller than JSON text representations.

- **Tokenized structure**  
  Each value begins with a type token, making parsing fast and predictable without the need for complex text parsing.

- **Rich numeric support**  
  Distinguishes between signed, unsigned, and floating-point numbers across multiple sizes, from **8 bits up to 64 bits**.

- **Minimalist compared to BSON**  
  Avoids rarely used types like `null`, `date`, `regex`, or `ObjectId`, focusing only on core data types.

- **Flexible string and array lengths**  
  Each string or array can explicitly declare its length (8, 16, 32, or 64 bits), reducing overhead for small values and allowing large ones when needed.

- **Hex-dump friendly**  
  Unlike JSON, YAD is designed to be **readable and understandable** when inspected with a hex editor or dumper.

---

## 📌 Use Cases

- Custom databases or storage engines.
- High-performance applications where JSON is too verbose and BSON is too heavy.
- Scenarios where deterministic, low-level parsing is required.
- Lightweight serialization for embedded systems.

---

## 🚀 Roadmap

- [ ] Reference parser/serializer implementation.
- [ ] Example comparisons with JSON and BSON (size and performance).
- [ ] Tooling for easier inspection and conversion.

---

## 📚 Definitions
YAD uses a system where each byte indicates one thing in the most optimized way. Below, I will explain the types and how they are handled.

### ❗ Important headers
YAD uses **special** binary values to mark the beginning or end of a “row.”
```
0xF0 => Version of YAN File, the next three uint8 indicates the version 
0xF1 => Start of a "row"
0xF2 => End of a "row"
```

### 📏 Length indicators
YAD uses 4 bits to indicate the expected length for each value. Below we show you what they are:
```
0xX1 => Indicates an 8-bit length
0xX2 => Indicates an 16-bit length
0xX3 => Indicates an 32-bit length
0xX4 => Indicates an 64-bit length
```

### 🔍 Types
YAD uses 4 bits to indicate the type. The existing types are shown below.
```
0x1X => Indicates an unsigned integer
0x2X => Indicates an signed integer
0x3X => Indicates a floating point number.
0x4X => Indicates a string in UTF-8
0x5X => Indicates an array
0x6X => Indicates the name of the “row”; works the same as the string.
0x7X => Indicates the name of the “key”; works the same as the string.
```

### 📝 Example
YAD Example File (human-readable)

| Byte(s)              | Meaning                                                                    |
|----------------------|----------------------------------------------------------------------------|
| F0 01 00 00          | Version header: F0 = version token, 01 00 00 = version 1.0.0               |
| F1                   | Start of row                                                               |
| 64 04 75 73 65 72    | Row name: 6X = row name type, 04 = length in 8-bit, bytes = "user"         |
| 71 02 69 64          | Key name: 7X = key type, 02 = length 8-bit, bytes = "id"                   |
| 11 2A                | Value: 11 = uint8 type, 2A = 42 (decimal)                                  |
| 71 04 6E 61 6D 65    | Key name: 7X = key type, 04 = length 8-bit, bytes = "name"                 |
| 41 05 4A 6F 68 61 6E | Value: 4X = string type, 05 = length 8-bit, 05 bytes long, UTF-8 = "Johan" |
| F2                   | End of row                                                                 |

Or in a raw file:
```
F0 01 00 00 F1 64 04 75 73 65 72 71 04 69 64 11 2A 71 04 6E 61 6D 65 41 05 4A 6F 68 61 6E F2
```