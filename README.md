# Flashpack - A Flash Card Desktop App

Flashpack is a passion project born out of the desire to create a studying tool that not only encompasses traditional flashcards, but also caters to the unique challenges of math-based questions.

## Features

- **Scriptable Cards**: One of Flashpack's key features is its ability to generate question cards with variable content through user-made scripts. This is particularly beneficial for subjects like math, where randomized variations of numeric answers aid in better understanding and prevent memorization of specific values.
- **AsciiMath Support**: Flashpack supports rendering complex math expressions through [AsciiMath](http://asciimath.org/), which allows for a more readable source text.

## Screenshots

TODO

## Installation

*No releases yet*

## Usage

To run this locally from source, first make sure to complete the [Tauri prerequesites](https://tauri.app/v1/guides/getting-started/prerequisites) and install the Tauri cli:

```
$ cargo install tauri-cli
```

Next, clone this repo and install the necessary npm modules:

```
$ git clone git@github.com:Cabidge/flashpack.git
$ cd flashpack
$ npm i
```

Once that's done, use the Tauri cli to run in debug mode:

```
$ cargo tauri dev
```

Or, build an executable in release mode:

```
$ cargo tauri build
$ cd ./src-tauri/target/release
$ ls
```
