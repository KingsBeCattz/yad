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
YAD uses **special** binary values to mark the beginning or end of a structure like a “row” or a “key.”
```md
0xF0 => Version of YAD file, the next four uint8 indicate major, minor, and patch version, followed by a beta flag
0xF1 => Start of a "row"
0xF2 => End of a "row"
0xF3 => Start of a "key"
0xF4 => End of a "key"
```

### 📏 Length indicators
YAD uses 8 bits (1 byte) to indicate the length for each value. The prefix byte also encodes the type of the value.
```
0xX1 => Indicates an 8-bit length
0xX2 => Indicates an 16-bit length
0xX3 => Indicates an 32-bit length
0xX4 => Indicates an 64-bit length
```

### 🔍 Types
YAD uses 4 bits in the prefix byte to indicate the type. The existing types are shown below.

#### 🔢 Numbers
In YAD a number is represented like this:
```md
0x11 0x11
```
That means "Unsigned integer of 8 bits" wich value is 0x11 (17)
Following this, exists 3 types of numbers:
```md
0x1X => Unsigned integer (uin8, uint16, etc.) 
0x2X => Signed integer 
0x3X => Floating point number 
```
> Note: The numbers are in big-endian format
#### ✏ Strings
In YAD, a string is represented like this:
```md
0x41 0x04 0x6E 0x61 0x6D 0x65
```
That means "A string wich length is less of u8, his length is of 0x04 Bytes", the next bytes is the text in UTF-8, in this case, is "name"
#### ❓ Booleans
YAD represents booleans like this:
```md
0x80 => False
0x81 => True
```
Any value equal or greater than 0x81 is considerated as `True`
#### 📃 Arrays
YAD represents arrays like this:
```md
0x51 0x01 0x41 0x04 0x6E 0x61 0x6D 0x65
```
That means: "An array with length is less of u8, his length is of one who is a String ..."
#### 🏷 Keys names and Rows names
These works same as Strings but they are only used for his context, on keys and rows

### 📝 Example
YAD Example File (human-readable)

| Byte(s)                    | Meaning                                                                 |
|----------------------------|-------------------------------------------------------------------------|
| F0 00 00 01 01             | Version header: F0 = version token, 00 00 01 = version 0.0.1, 01 = beta |
| F1                         | Start of row                                                            |
| 61 05 6A 6F 68 61 6E       | Row name: 61 = row name type, 05 = length, bytes = "johan"              |
| F3                         | Start of key                                                            |
| 71 04 6E 61 6D 65          | Key name: 71 = key type, 04 = length, bytes = "name"                    |
| 41 05 4A 6F 68 61 6E       | Value: 41 = string type, 05 bytes long, UTF-8 = "Johan"                 |
| F4                         | End of key                                                              |
| F3                         | Start of key                                                            |
| 71 03 61 67 65             | Key name: 71 = key type, 03 = length, bytes = "age"                     |
| 11 11                      | Value: 11 = uint8 type, 11 = 17 (decimal)                               |
| F4                         | End of key                                                              |
| F2                         | End of row                                                              |
| F1                         | Start of row                                                            |
| 61 07 73 69 6C 65 6E 63 65 | Row name: 61 = row name type, 07 = length, bytes = "silence"            |
| F3                         | Start of key                                                            |
| 71 04 6E 61 6D 65          | Key name: 71 = key type, 04 = length, bytes = "name"                    |
| 41 07 53 69 6C 65 6E 63 65 | Value: 41 = string type, 07 bytes long, UTF-8 = "Silence"               |
| F4                         | End of key                                                              |
| F3                         | Start of key                                                            |
| 71 03 61 67 65             | Key name: 71 = key type, 03 = length, bytes = "age"                     |
| 11 11                      | Value: 11 = uint8 type, 11 = 17 (decimal)                               |
| F4                         | End of key                                                              |
| F2                         | End of row                                                              |

Or in a raw file:
```
0xF0 0x0 0x0 0x1 0x1 0xF1 0x61 0x5 0x6A 0x6F 0x68 0x61 0x6E 0xF3 0x71 0x4 0x6E 0x61 0x6D 0x65 0x41 0x5 0x4A 0x6F 0x68 0x61 0x6E 0xF4 0xF3 0x71 0x3 0x61 0x67 0x65 0x11 0x11 0xF4 0xF2 0xF1 0x61 0x7 0x73 0x69 0x6C 0x65 0x6E 0x63 0x65 0xF3 0x71 0x4 0x6E 0x61 0x6D 0x65 0x41 0x7 0x53 0x69 0x6C 0x65 0x6E 0x63 0x65 0xF4 0xF3 0x71 0x3 0x61 0x67 0x65 0x11 0x11 0xF4 0xF2
```