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
