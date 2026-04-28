# Propsolve

### Describe propositional logic formulas in a simple language and have a SAT solver solve them

**Motivated by** the fact that common SAT/SMT solver interfaces like the [SMT-LIB language](https://smt-lib.org/language.shtml) take unnecessarily long to use when just trying to quickly describe and check a small propositional logic formula/problem.

```
$ propsolve --eval "a & b -> c"
formula is satisfiable, e.g. with:

a := 1
b := 1
c := 1

formula is also unsatisfiable, e.g. with:

a := 1
b := 1
c := 0
```

## Features

- Automatic check for satisfiability, unsatisfiability and tautology
- Transpile to [SMT-LIB language](https://smt-lib.org/language.shtml) to solve with any compatible solver using `--print`
- Read formula from file or command line argument
    - See `propsolve --help`
- Fancy parser errors using [ariadne](https://docs.rs/ariadne/latest/ariadne/)

# Language Description

| Syntax           | Semantics                                              |
|------------------|--------------------------------------------------------|
| `true` / `false` | Boolean value constants                                |
| `!a`             | Negation / Not                                         |
| `a & b`          | Conjunction / And                                      |
| `a \| b`         | Disjunction / Or                                       |
| `a -> b`         | Implication                                            |
| `a == b`         | Equivalence (Equal)                                    |
| `a != b`         | Negated Equivalence (Not Equal)                        |
| `a ^ b`          | Exclusive Or (Xor)                                     |
| `a; b`           | Multiple formulas which are all asserted to be true |
| `// comment`     | Single-line comment                                    |
| `/* comment */`  | Multi-line comment                                     |

- All used identifiers are treated as Boolean variables
    - except for the constants `true` and `false` 
- All formulas are asserted to be true
- All operators with multiple operands are left-associative
    - meaning `a & b & c == (a & b) & c` 
- Operator precedence is the following (in descending order):
    - `!`
    - `&`, `|`, `^`
    - `->`
    - `==`, `!=`

An example formula (text file):
```c
N -> M; // comment
(O & L) | (!O & !L);
(M & N) | (N & A) | (A & M);
/* a multi-line
   comment      */
!(M & N & A);
(S | C) ^ A == (!(S | C) | !A) & (S | C | A);
!B & !T;
//A;
```

# Installation

## Download and run a prebuilt binary...

...from the [latest release](https://github.com/julius-boettger/propsolve/releases/latest) (if available for your platform)

## Build and run from source

### Using [Nix Flakes](https://wiki.nixos.org/wiki/Flakes)
```sh
# option 1: fully automatic
nix run github:julius-boettger/propsolve
# option 2: fetch source, build, run
git clone https://github.com/julius-boettger/propsolve
cd propsolve
nix build
./result/bin/propsolve
```

### Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
Prerequisite is having [z3](https://github.com/Z3Prover/z3) installed.
```sh
# fetch the source
git clone https://github.com/julius-boettger/propsolve
cd propsolve
# build
cargo build --release
# run
./target/release/propsolve[.exe]
```
