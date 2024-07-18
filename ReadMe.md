# Low-Level Design of Backend Systems with Rust

Welcome to the Low-Level Design of Backend Systems with Rust repository! This comprehensive repository is dedicated to learning and implementing the low-level design of back-end systems using Rust. Each step and component in this repository is designed to enhance your understanding of Rust and back-end development principles.

## Table of Contents

- [Low-Level Design of Backend Systems with Rust](#low-level-design-of-backend-systems-with-rust)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Setup and Installation](#setup-and-installation)
  - [Folder Structure](#folder-structure)
    - [lld-1-rust](#lld-1-rust)
      - [0-basics](#0-basics)
      - [1-oops](#1-oops)
      - [2-concurrency](#2-concurrency)
      - [3-advanced-concepts](#3-advanced-concepts)
      - [4-web-server](#4-web-server)
    - [lld-2-design](#lld-2-design)
      - [0-SOLID](#0-solid)
      - [1-Design Patterns](#1-design-patterns)
    - [lld-3-case studies](#lld-3-case-studies)
  - [Contributing](#contributing)
  - [License](#license)

## Introduction

This repository is designed to provide a structured approach to learning the low-level design principles of back-end systems using Rust. It covers fundamental concepts, advanced topics, and practical projects to help you build robust and efficient back-end systems.

## Setup and Installation

To get started with this repository, follow these steps:

1. **Clone the repository**:
    ```sh
    git clone https://github.com/smbsp/lld-backend-rust.git
    cd lld-backend-rust
    ```

2. **Install Rust**:
    Follow the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust.

3. **Install dependencies**:
    ```sh
    cargo build
    ```

4. **Run the project**:
    ```sh
    cargo run
    ```

## Folder Structure

### lld-1-rust

#### 0-basics

- **Objective**: Introduction to Rust programming.
- **Details**: Setting up the Rust environment, understanding Rust syntax, variables, data types, and basic control flow.
  - **my_proc_macro**: Custom procedural macros in Rust.
  - **src**:
    - **m1_enums.rs**: Understanding Rust enums and their use cases.
    - **m2_structs.rs**: Defining and using structs in Rust.
    - **m3_traits.rs**: Implementing and using traits for polymorphism.
    - **m4_polymorphism.rs**: Polymorphism in Rust using traits and enums.
    - **m5_lifetimes.rs**: Understanding and managing lifetimes in Rust.
    - **m6_patterns.rs**: Using pattern matching in Rust.
    - **m7_async.rs**: Writing asynchronous code with async/await.
    - **m8_collections.rs**: Using collections like Vec, HashMap, etc.
    - **m9_decl_macros.rs**: Declaring and using macros.
    - **m10_proc_macros.rs**: Creating and using procedural macros.
    - **m11_smart_pointers.rs**: Using smart pointers like Box, Rc, and Arc.
    - **m12_concurrency.rs**: Handling concurrency with threads and async.

#### 1-oops

- **Objective**: Object-oriented programming in Rust.
- **Details**: Implementing OOP principles like encapsulation, inheritance, and polymorphism using Rust's unique features.

#### 2-concurrency

- **Objective**: Concurrency in Rust.
- **Details**: Exploring Rust’s concurrency model, using threads, async/await, and concurrency primitives like Mutex and channels.

#### 3-advanced-concepts

- **Objective**: Advanced Rust concepts.
- **Details**: Exploring advanced topics like unsafe code, FFI, memory management, and performance optimization.

#### 4-web-server

- **Objective**: Building a web server in Rust.
- **Details**: Creating a web server using frameworks like Actix-web or Rocket, handling routing, middleware, and request/response life cycles.

### lld-2-design

#### 0-SOLID

- **Objective**: SOLID principles in Rust.
- **Details**: Implementing SOLID design principles (Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, and Dependency Inversion) in Rust applications.

#### 1-Design Patterns

- **Objective**: Common design patterns in Rust.
- **Details**: Implementing design patterns like Singleton, Factory, Strategy, Observer, etc., in Rust.

### lld-3-case studies

- **Objective**: Real-world case studies.
- **Details**: Analyzing and implementing solutions for real-world problems and applications using Rust.

## Contributing

Contributions are welcome! If you have improvements or additional features to add, please submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
