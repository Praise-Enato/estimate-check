# Overview

As a software engineer, I want to be able to pick up a new language and write useful programs in it. Rust was new to me, and it works differently from the languages I already know. It has no garbage collector. Instead, every value has one owner, and the compiler checks how each value is borrowed before it will build the program. With this project I wanted to learn those rules, along with Rust's basic syntax: variables that are immutable unless marked `mut`, expressions, `match`, loops, functions and structs.

Estimate Check is a small command-line program for checking how good I am at estimating time. It shows a menu with three choices: add a task, list my tasks, or quit. When you add a task, it asks for the task name, the hours you estimated, and the hours it really took. It checks every answer and asks again if the name is empty or the hours are not a number between 0 and 1000. When you list your tasks, it shows how far off each estimate was as a percentage, for example "estimated 3.0 h, actual 4.5 h, 50% over". It then shows the total hours, the overall percentage, how many tasks went over, and a short tip.

I wrote this software because estimating how long my work will take is a habit I am trying to improve. Building a tool for it gave me a real reason to learn Rust's ownership rules: the list of tasks is borrowed with `&` when it is only read, and with `&mut` when a new task is added.

[Software Demo Video](PASTE_YOUR_VIDEO_LINK_HERE)

# Development Environment

I wrote the program in Visual Studio Code with the rust-analyzer extension on a MacBook. I installed Rust with rustup and used Cargo to create, build and run the project. I used Git and GitHub for version control.

The program is written in Rust and only uses the standard library (`std::io` for reading what the user types). The tasks are stored in a `Vec<Task>`, where `Task` is a struct holding the name, the estimated hours and the actual hours.

### How to run it

```
cargo run
```

# Useful Websites

* [The Rust Programming Language (the Rust Book)](https://doc.rust-lang.org/book/)
* [Rust Book Chapter 2: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
* [Rust Book Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
* [Rust Book Chapter 8.1: Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [Rust Standard Library: std::io::Stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html)

# Future Work

* Save the tasks to a file so they are still there the next time the program runs.
* Let the user edit or delete a task that was typed in wrong.
* Group tasks by type (for example studying, coding, writing) to see which kind of work I estimate worst.
* Move the percentage code into an `impl` block on `Task` once I am more comfortable with structs.
