# Rust Primitive Data Types

Rust is a statically-typed language, which means it must know the types of all variables at compile time. However, the compiler can often infer the type based on the value and how we use it. Rust provides several built-in primitive data types to represent various kinds of data.

Here's an overview of Rust's primitive data types:

## 1. Scalar Types

Scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

### a. Integers

Integers are whole numbers. Rust's integer types are categorized by whether they are signed (can be positive or negative) or unsigned (only positive) and by their size (number of bits they occupy).

*   **Signed Integers:** `i8`, `i16`, `i32`, `i64`, `i128`
    *   The `i` stands for "integer," and the number indicates the number of bits.
    *   `isize`: Pointer-sized signed integer. Its size depends on the architecture of the computer (e.g., 32 bits on a 32-bit architecture, 64 bits on a 64-bit architecture). Used for indexing collections.
*   **Unsigned Integers:** `u8`, `u16`, `u32`, `u64`, `u128`
    *   The `u` stands for "unsigned."
    *   `usize`: Pointer-sized unsigned integer. Its size depends on the architecture. Used for indexing collections.

**Default Type:** If you don't specify an integer type, Rust defaults to `i32`.

**Example:**
```rust
let x: i32 = 42; // Explicit type annotation
let y = 100;     // i32 by default
let z: u64 = 1_000_000; // Underscores can be used for readability
let index: usize = 5;
```

### b. Floating-Point Numbers

Rust has two floating-point types:

*   `f32`: Single-precision float (32-bit).
*   `f64`: Double-precision float (64-bit).

**Default Type:** Rust defaults to `f64` because it offers more precision.

**Example:**
```rust
let pi: f32 = 3.14;
let e = 2.71828; // f64 by default
```

### c. Booleans

The Boolean type has two possible values: `true` and `false`. It is specified with the `bool` keyword.

**Example:**
```rust
let is_rust_fun: bool = true;
let is_raining = false; // bool by default
```

### d. Characters

Rust's `char` type is a single Unicode scalar value, meaning it can represent a lot more than just ASCII. It's four bytes in size.

**Example:**
```rust
let initial: char = 'R';
let emoji = '😂';
```

## 2. Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### a. Tuples

Tuples are a way to group together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

**Example:**
```rust
let person_data: (String, i32, f64) = (String::from("Alice"), 30, 1.75);

// Accessing tuple elements by destructuring
let (name, age, height) = person_data;
println!("Name: {}, Age: {}, Height: {}", name, age, height);

// Accessing tuple elements by index
let favorite_color = person_data.0;
let favorite_number = person_data.1;
```

### b. Arrays

Arrays are also fixed-size collections, but all elements in an array must have the same type. Arrays are useful when you want your data allocated on the stack rather than the heap, or when you want to ensure you always have a fixed number of elements.

**Example:**
```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 i32 integers
let same_value = [3; 5]; // [3, 3, 3, 3, 3]

// Accessing array elements
let first_number = numbers[0];
let second_number = numbers[1];
```

### c. Slices (Reference Type)

While not a "primitive type" in the same way as integers or booleans, slices are a fundamental reference type that allows you to refer to a contiguous sequence of elements in a collection (like an array or `Vec`) without copying the entire collection. They are often considered alongside primitive types due to their fundamental nature.

**Example:**
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..4]; // This creates a slice: &[2, 3, 4]
```

Understanding these primitive data types is crucial for writing efficient and correct Rust code. The strong typing system helps the compiler catch many errors at compile time, leading to more reliable programs.