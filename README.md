# Rust CLI To-Do App 🦀

A simple command-line based To-Do application written in Rust.

This project is intended as a **learning project for Rust**, focusing on understanding how real programs manage data and state.

## Why this project?

This project is great for learning Rust because it covers:

- Basic **CRUD operations** (Create, Read, Update, Delete)
- Working with **structs** and **vectors**
- Understanding **ownership, borrowing, and mutability**
- Reading from and writing to files
- Using **JSON** for data persistence
- Converting Rust data structures to and from JSON using `serde`

By building this, you get hands-on experience with how Rust handles memory safety while still writing practical, real-world code.

## Features

- Add tasks
- List tasks
- Mark tasks as completed
- Edit task title and status
- Delete tasks
- Persistent storage using a JSON file
- Automatic ID reassignment after deletion

## Usage

```bash
cargo run add "Buy milk"
cargo run list
cargo run done 1
cargo run edit 1 "Buy milk and bread" true
cargo run delete 1
