# Rust Programming Language - Learning Repository

![Rust Logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

Welcome to the Rust Learning Repository! This repo contains notes and examples to help you understand key concepts of Rust, a systems programming language known for its memory safety, concurrency, and performance. 🚀

## Key Concepts

- **Memory Management:** 
  - **Ownership:** Rust uses ownership rules to manage heap data without needing a garbage collector. Each value has a single owner, and the value is dropped when the owner goes out of scope. 🗃️
  - **Borrowing:** Allows you to reference data without transferring ownership. Mutable references ensure no other references can be active simultaneously to prevent race conditions. 🔄
  
- **Data Types:**
  - **Scalar Types:** Integer, floating-point numbers. ➕
  - **Compound Types:** Array, tuple, string, vector. 🔢
  - **Mutability:** By default, data types are immutable. Use the `mut` keyword to make them mutable. 🔄

- **Functions:**
  - Return types are specified using the `->` arrow. ⬅️

- **Syntax Conventions:**
  - **Naming:** Rust uses `snake_case` for variable names and functions. CamelCase is used for types. 🐍
  - **Constants:** Must be in uppercase and specify the data type. 🔡

- **Memory Management Approaches:**
  - **Control First Approach:** Used in languages like C/C++ where pointers manage memory. 🧭
  - **Safety First Approach:** Used in languages like Python/Java with garbage collection. 🧹
  - **Ownership Model:** Rust’s unique approach where the ownership of data is strictly controlled. 🔒

- **Errors and Debugging:**
  - Be aware of errors related to ownership transfers and mutable references. ⚠️

- **Concepts Explained:**
  - **Borrowing:** Passing references to avoid ownership transfer. 🔗
  - **Shadowing:** Redeclaring a variable with the same name, treating it as a new variable. 🌠
  - **Match:** Pattern matching for control flow. 🔄
  - **Vectors:** Dynamic arrays in Rust. 📈
  - **Type Inference:** Rust can infer types based on the context. 🧠

## Examples

- **Ownership and Borrowing:** Examples to demonstrate ownership transfer and borrowing. 📝
- **Error Handling:** Common errors and their resolutions. 🔍
- **Data Types and Structures:** Examples of scalar and compound types, arrays, and vectors. 📚

## Setup and Usage

1. **Install Rust:** Follow the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started) to install Rust and Cargo. ⚙️
2. **Run Examples:** Use `cargo run` to execute Rust programs. ▶️
3. **Build Projects:** Use `cargo build` to compile projects. 🛠️

## Contributing

Feel free to contribute by adding more examples or improving existing notes. Please follow Rust's conventions and best practices. 🤝

---

Happy coding with Rust! 🌟

