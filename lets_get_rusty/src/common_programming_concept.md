# Variables and Mutability
[Page location in linux system](~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch03-01-variables-and-mutability.html)
## Intro
+ By default, **Variables** are immutable
+ **Immutability** means you can't change **assigned value** of a variable
## Why does Rust encourage you to write code in a way that favors **immutability**
+ Sometimes, in a **part of a code**, **variable** is **not changeble** but in the other part,it can  
=> The Compiler and coder **cant not track the error** => Should code with **immutable variables**
# Constants
+ Like **immutable variables** but with some **key differences**
+ Cant add `mut` when declared **constants**
+ declare like this
`const HOURS_IN_A_DAY: i32`
+ contants **naming convention** is **all uppercase**