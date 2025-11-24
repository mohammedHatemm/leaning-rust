# Compound Data Types in Rust

This document provides a comprehensive overview of compound data types in Rust, including explanations, syntax, best practices, and common misconceptions.

## 1. Tuples

Tuples are a general way of grouping together a number of values with a variety of types into one compound type.

### Explanation and Syntax

A tuple is a fixed-size collection of values that can have different types. Once declared, a tuple's size cannot grow or shrink.

**Syntax:**
```rust
// A tuple with three elements of different types
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Destructuring a tuple to get individual values
let (x, y, z) = tup;

// Accessing tuple elements by index
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
```

### Good vs. Bad Practices

| Good Practice ✅                                                              | Bad Practice ❌                                                                 |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| Use for small, fixed-size collections of related but different-typed data.    | Using for long lists of elements (a struct might be better).                    |
| Destructuring to access elements, which makes the code more readable.         | Accessing elements by index (`tup.0`, `tup.1`) can be less clear than destructuring. |
| Returning multiple values from a function.                                    | Overusing tuples where a `struct` would provide more meaning with named fields. |

### Right vs. Wrong Explanations

| Right Explanation 👍                                      | Wrong Explanation 👎                                                              |
| --------------------------------------------------------- | --------------------------------------------------------------------------------- |
| A simple, anonymous compound type for grouping values.    | It's like a list in Python (Python lists are dynamically sized and can be modified). |
| A fixed-size collection of heterogeneous values.          | You can add or remove elements from a tuple after it's created.                   |

## 2. Arrays

Arrays are another way to have a collection of multiple values. Unlike a tuple, every element of an array must have the same type.

### Explanation and Syntax

An array is a fixed-size collection of values of the same type. Arrays in Rust are stored on the stack rather than the heap.

**Syntax:**
```rust
// An array of 5 integers
let a = [1, 2, 3, 4, 5];

// An array with explicit type and size
let a: [i32; 5] = [1, 2, 3, 4, 5];

// An array with all elements set to the same value
let a = [3; 5]; // is the same as let a = [3, 3, 3, 3, 3];

// Accessing array elements by index
let first = a[0];
let second = a[1];
```

### Good vs. Bad Practices

| Good Practice ✅                                                              | Bad Practice ❌                                                                 |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| Use when you know the number of elements will not change.                     | Trying to add or remove elements (use a `Vec<T>` for dynamic collections).      |
| Good for stack-allocated data, which can be faster than heap-allocated data.  | Using an array when you need a collection that can grow or shrink.              |

### Right vs. Wrong Explanations

| Right Explanation 👍                                      | Wrong Explanation 👎                                                              |
| --------------------------------------------------------- | --------------------------------------------------------------------------------- |
| A fixed-size list where every element must have the same type. | It's a dynamic list of elements.                                                  |
| A contiguous block of memory for elements of the same type. | Arrays and slices (`&[T]`) are the same thing (a slice is a view into an array). |

## 3. Structs

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

### Explanation and Syntax

Structs are like blueprints for creating instances of a custom type. They are composed of named fields.

**Syntax:**
```rust
// Defining a struct
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// Creating an instance of a struct
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// Accessing struct fields
println!("{}", user1.username);
```

### Good vs. Bad Practices

| Good Practice ✅                                                              | Bad Practice ❌                                                                 |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| Use to model custom data types with meaningful field names.                   | Creating structs with too many fields (consider breaking them down).            |
| Use `impl` blocks to define methods associated with a struct.                 | Using very generic field names that don't describe the data.                    |

### Right vs. Wrong Explanations

| Right Explanation 👍                                      | Wrong Explanation 👎                                                              |
| --------------------------------------------------------- | --------------------------------------------------------------------------------- |
| A way to create more complex data types with named fields. | It's just a collection of variables.                                              |
| A template for creating instances of a custom data type.  | Structs and hashmaps are the same (hashmaps are for key-value storage).           |

## 4. Enums

Enums, or enumerations, are a type that can have one of several possible variants.

### Explanation and Syntax

Enums allow you to define a type by enumerating its possible variants. Each variant can optionally have data associated with it.

**Syntax:**
```rust
// Defining an enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Using an enum
let msg = Message::Write(String::from("hello"));

// Using a match expression to handle enum variants
match msg {
    Message::Quit => {
        println!("The Quit variant has no data to destructure.");
    }
    Message::Move { x, y } => {
        println!(
            "Move in the x direction {} and y direction {}",
            x, y
        );
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
        println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        )
    }
}
```

### Good vs. Bad Practices

| Good Practice ✅                                                              | Bad Practice ❌                                                                 |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| Use `match` to handle all possible variants, ensuring exhaustive checking.    | Using `if let` when you should handle all variants.                             |
| Use enums to model a state machine or a set of choices.                       | Not using the data associated with a variant.                                   |

### Right vs. Wrong Explanations

| Right Explanation 👍                                      | Wrong Explanation 👎                                                              |
| --------------------------------------------------------- | --------------------------------------------------------------------------------- |
| A way to define a type that can be one of a few different variants. | It's just a list of constants.                                                    |
| A type that can hold different kinds of values at different times. | Enums are only for simple, valueless flags.                                     |

## 5. String vs. &str (String Slice)

Rust has two main types of strings: `String` and `&str`. Understanding the difference is crucial for writing efficient and correct Rust code.

### Explanation and Syntax

- **`String`**: A growable, mutable, owned, UTF-8 encoded string type. When you create a `String`, it allocates memory on the heap. This means you can modify it, such as by adding more text to it. Because it's owned, when the `String` goes out of scope, its memory is automatically freed.

- **`&str`**: Also known as a "string slice," it's an immutable reference to a sequence of UTF-8 bytes. A `&str` is a "view" into string data owned by someone else. It can point to a part of a `String`, a string literal embedded in the binary, or any other string data. String literals (`"hello"`) are of type `&'static str`.

**Syntax:**
```rust
// A string literal, which is a slice (&str)
let s1 = "hello world";

// A String, created from a string literal
let mut s2 = String::from("hello");

// You can modify a String
s2.push_str(", world!");

// You can create a slice from a String
let slice_of_s2 = &s2[0..5]; // slice_of_s2 is "hello"

println!("{}", s2); // prints "hello, world!"
```

### Mutability in Rust

By default, variables in Rust are **immutable**. This is one of Rust's core principles to enforce safety and prevent unintended changes to data.

- To make a variable mutable, you must use the `mut` keyword.
- Rust as a language is not "mutable" or "immutable"; it provides the tools for you to choose the right level of mutability for your needs.

```rust
let immutable_var = 5;
// immutable_var = 6; // This would cause a compile-time error!

let mut mutable_var = 10;
mutable_var = 11; // This is allowed
```

| Good Practice ✅                                                              | Bad Practice ❌                                                                 |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| Prefer immutable variables (`let`) unless you explicitly need to change them. | Making all variables mutable (`let mut`) by default.                            |
| Use `&str` for function arguments to be flexible and accept any string type.  | Using `String` for function arguments when you only need to read the string.    |

### Stack vs. Heap

Memory in a Rust program is managed in two primary locations: the stack and the heap.

- **Stack**:
  - **How it works**: Last-In, First-Out (LIFO). Very fast because it just involves moving a single pointer (the "stack pointer").
  - **What goes here**: Data with a known, fixed size at compile time. This includes primitive types (like `i32`, `bool`), tuples, arrays, and references (`&str`, `&i32`).
  - **Analogy**: A stack of plates. You can only add or remove plates from the top.

- **Heap**:
  - **How it works**: When you need to store data but don't know the size at compile time, or if the size might change, you request a certain amount of space from the operating system. The OS finds an empty spot (the "heap") and returns a **pointer** to that location. This is slower than the stack.
  - **What goes here**: Data that can grow or change size, like `String` and `Vec<T>`.
  - **Analogy**: A restaurant seating. You ask for a table for a certain number of people, and the host finds a spot for you.

| Type      | Stored on... | Why?                                                              |
| --------- | ------------ | ----------------------------------------------------------------- |
| `i32`     | Stack        | Fixed size known at compile time.                                 |
| `&str`    | Stack        | It's a reference (a pointer and a length), which has a fixed size. |
| `String`  | Heap         | The actual text data is on the heap because it can grow.          |

### References vs. Pointers

- **References (`&`)**:
  - This is Rust's safe way of "borrowing" data without taking ownership.
  - The Rust compiler's **borrow checker** guarantees that references will **always** point to valid data. You cannot have a "null reference" or a reference to data that has been freed.
  - **Syntax**: `&` (borrow) and `*` (dereference).

- **Raw Pointers (`*const T` and `*mut T`)**:
  - These are like pointers in C/C++. They are just memory addresses.
  - You can have null pointers, pointers to invalid memory, or pointers to data of the wrong type.
  - Working with raw pointers is considered **`unsafe`** in Rust. You must wrap any code that uses them in an `unsafe` block, signaling to the compiler that you are manually upholding memory safety.
  - **Use Case**: Primarily for interoperating with other languages (like C) or for building low-level abstractions.

| Right Explanation 👍                                      | Wrong Explanation 👎                                                              |
| --------------------------------------------------------- | --------------------------------------------------------------------------------- |
| A `&str` is an immutable view into string data.           | A `&str` is a string that can't be changed. (The binding can be mutable).         |
| A `String` is an owned, heap-allocated string.            | `String` and `&str` are interchangeable. (They are not; conversion is needed).    |
| References are guaranteed by the compiler to be valid.    | References and pointers are the same thing in Rust.                               |

