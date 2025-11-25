# Functions in Rust

This document provides a comprehensive overview of functions in Rust, including their definition, usage, and related concepts.

## Table of Contents

- [What is a Function?](#what-is-a-function)
- [Defining and Calling Functions](#defining-and-calling-functions)
- [Function Parameters](#function-parameters)
- [Function Return Values](#function-return-values)
- [Statements vs. Expressions](#statements-vs-expressions)
- [Variable Scope: Inside vs. Outside `main`](#variable-scope-inside-vs-outside-main)
- [Important Built-in Macros and Functions](#important-built-in-macros-and-functions)
- [Example: BMI Calculator Function](#example-bmi-calculator-function)

---

## What is a Function?

Functions are a fundamental building block in Rust. They are self-contained blocks of code that perform a specific task. Functions help in organizing code into reusable and manageable pieces.

## Defining and Calling Functions

In Rust, you define a function using the `fn` keyword, followed by the function name, a set of parentheses `()`, and a block of code enclosed in curly braces `{}`.

To call a function, you simply use its name followed by parentheses.

```rust
fn main() {
    println!("Hello from main!");
    another_function(); // Calling another_function
}

fn another_function() {
    println!("Hello from another function!");
}
```

## Function Parameters

Functions can take inputs, which are called parameters. You must declare the type of each parameter.

```rust
fn main() {
    print_value(42);
}

fn print_value(x: i32) {
    println!("The value is: {}", x);
}
```

## Function Return Values

Functions can return a value to the code that calls them. The return type is specified after an arrow `->`. The last expression in the function will be used as the return value. Alternatively, you can use the `return` keyword to return a value early.

```rust
fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // This is an expression, and it's the return value
}
```

---

## Statements vs. Expressions

This is a crucial concept in Rust.

- **Statements** are instructions that perform an action but do not return a value. For example, `let x = 5;` is a statement. Function definitions are also statements.

- **Expressions** evaluate to a value. For example, `5 + 3` is an expression that evaluates to `8`. A function call is an expression. A block of code `{}` can also be an expression if it evaluates to a value.

```rust
fn main() {
    // `let y = 6;` is a statement. It doesn't return a value.
    // You cannot do `let x = (let y = 6);`

    let y = {
        let x = 3;
        x + 1 // No semicolon here, so this is an expression
    }; // This block is an expression that returns 4, so y will be 4.

    println!("The value of y is: {}", y);
}
```

---

## Variable Scope: Inside vs. Outside `main`

The location of a variable declaration determines its scope (where it can be accessed).

### Variables Inside a Function

Variables declared inside a function (like `main`) are local to that function. They are created when the function is called and destroyed when the function exits.

```rust
fn main() {
    let local_variable = "I'm local to main";
    println!("{}", local_variable);
}

fn another_function() {
    // This would cause an error:
    // println!("{}", local_variable);
}
```

### Variables Outside a Function (Global Scope)

To define variables that are accessible from any part of the program, you can use `const` or `static`.

- **`const`**: Used for defining compile-time constants. The value must be known at compile time. They are always immutable.

- **`static`**: Used for variables that have a fixed memory location. They can be mutable (but requires an `unsafe` block to modify).

```rust
// Global constant
const PI: f32 = 3.14159;

// Global static variable
static mut COUNTER: u32 = 0;

fn main() {
    println!("PI is {}", PI);

    // Modifying a static mutable variable requires an unsafe block
    unsafe {
        COUNTER += 1;
        println!("Counter is {}", COUNTER);
    }
}
```

---

## Important Built-in Macros and Functions

Rust's standard library provides many useful functions and macros. Macros are a way of writing code that writes other code (metaprogramming). You can often recognize them by the `!` at the end of their name.

- **`println!()`**: Prints text to the console.
- **`vec![]`**: A convenient macro to create a `Vec<T>` (a growable list).
- **`assert_eq!()` / `assert!()`**: Used in tests to check for equality or a boolean condition. The program will panic if the assertion fails.
- **`.to_string()`**: A common method to convert a value to a `String`.
- **`.len()`**: A method on collections (like strings, vectors, arrays) to get their length.
- **`.iter()`**: A method to create an iterator over a collection, which allows you to loop over its elements.

---

## Example: BMI Calculator Function

BMI (Body Mass Index) is a measure used to gauge whether a person has a healthy body weight for their height. The formula is:

**BMI = weight (kg) / (height (m))^2**

Here is a Rust function to calculate BMI:

```rust
fn calculate_bmi(weight_kg: f32, height_m: f32) -> f32 {
    // Check for division by zero to avoid a panic
    if height_m <= 0.0 {
        return 0.0;
    }
    weight_kg / (height_m * height_m)
}

fn main() {
    let weight = 75.0; // 75 kg
    let height = 1.8;  // 1.8 meters

    let bmi = calculate_bmi(weight, height);

    println!("Weight: {} kg", weight);
    println!("Height: {} m", height);
    println!("Your BMI is: {:.2}", bmi); // Print with 2 decimal places

    if bmi < 18.5 {
        println!("You are underweight.");
    } else if bmi < 25.0 {
        println!("You have a normal weight.");
    } else if bmi < 30.0 {
        println!("You are overweight.");
    } else {
        println!("You are obese.");
    }
}
```

This example demonstrates:
- Defining a function `calculate_bmi` that takes parameters and returns a value.
- Calling this function from `main`.
- Using the return value in conditional logic.
