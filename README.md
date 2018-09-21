# Async
Modern Shell written in Rust for golang

The name Async comes from the fact, the project sprung out from me trying to implement to a simple asynchronous system interrupt along side an IO operation in rust (Yeah it's a pretty crazy story)

Inspired by [this project](https://github.com/psinghal20/rush)

The main idea of the project is to be able to provide a development environment for 
- Go Projects
- Python projects
- Rust Projects

for enthusiastic coders who are new to the following languages. This is because each language project require it's own special environment.

**Features to be included**
- support for normal shell scripting so users won't have the necessity to shift back when they are done using it
- special shell commands for each language in terms of project development, testing and CI making
  

Contributions are always welcome, even if you have zero experience towards the language.

### Running the project 

In order to run this shell first clone this repository to your local machine 

```console
git clone https://github.com/sahitpj/Async
```

then change to the current directory and run cargo

```console
cd Async
cargo run
```

