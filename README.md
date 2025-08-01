# inkspect

`inkspect` is a powerful CLI tool for refining and generating text and code using Large Language Models (LLMs). It's designed to streamline your prompting workflow, keeping you in your terminal and integrated with your favorite tools.

## The Motivation: Stop Juggling, Start Creating

Does this workflow sound familiar?

1.  You start drafting a prompt in a web-based chatbot.
2.  You copy the text and paste it into a local file in your favorite editor for serious refinement.
3.  You tweak and iterate, getting it just right.
4.  Finally, you copy the finished prompt and paste it *again* into its final destination—a script, an application, or another AI agent.

This constant context-switching between your browser, your editor, and your terminal is inefficient and breaks your creative flow.

`inkspect` was built to solve this. It brings the power of LLMs directly to your command line, allowing you to work on prompts in your editor of choice and use them immediately, all without leaving your development environment.

## Features

*   **Multiple LLM Backends:** Supports Gemini (default) and Claude.
*   **Flexible Input:** Provide input via a command-line flag or your favorite text editor.
*   **Powerful Prompt Styles:** Use pre-defined prompt styles to get the exact output you need, from refining text to generating code.
*   **Intelligent Prompting:** A global system prompt ensures LLM outputs are direct and clean, with an option to disable it for full control.
*   **Customizable:** Configure API keys, default providers, and custom prompt styles in a simple TOML file.
*   **Verbose Logging:** A `--verbose` mode for debugging and inspecting the full interaction with the LLM.
*   **Easy Setup:** A `setup` command to get you started in seconds.

## Installation

### Recommended: From Crates.io

The easiest way to install `inkspect` is directly from `crates.io` using `cargo`.

```bash
cargo install inkspect
```

This will download, compile, and install the `inkspect` binary in your Cargo home directory (`~/.cargo/bin`), making it available from anywhere in your terminal.

### From Source

If you want to build the latest development version, you can build it from the source code.

1.  **Install Rust:** If you don't already have it, install the Rust toolchain from [rust-lang.org](https://www.rust-lang.org/tools/install).
2.  **Clone and Build:**
    ```bash
    git clone https://github.com/your-repo/inkspect.git
    cd inkspect
    cargo build --release
    ```
3.  **Install (Optional):** For easy access, you can copy the executable to a directory in your system's `PATH`.
    ```bash
    sudo cp target/release/inkspect /usr/local/bin/
    ```

## Configuration

The easiest way to get started is with the interactive `setup` command. It will create the configuration file for you at the correct location (`~/.config/inkspect/inkspect.toml`) and prompt you for your API keys.

```bash
inkspect setup
```

You can also provide a custom configuration path for any command using the `--config` flag.

## Usage

The basic command structure is `inkspect [OPTIONS] <COMMAND>`.

### Commands

#### `optimize`

This is the main command for processing text with an LLM.

*   **From an Editor (Default):**
    ```bash
    inkspect optimize
    ```
    This will open your default text editor (`$EDITOR`). Write your prompt, save, and close the file.

*   **From an Inline String:**
    ```bash
    inkspect optimize --input "your prompt here"
    ```

*   **Saving to a File:**
    ```bash
    inkspect optimize --input "your prompt here" --output my_file.txt
    ```

#### `list-styles`

Lists all the available prompt styles from your configuration file.

```bash
inkspect list-styles
```

#### `list-models`

Lists the available models from a specific provider.

```bash
inkspect list-models --provider gemini
```

#### `setup`

Runs the interactive setup to create your configuration file.

```bash
inkspect setup
```

### The System Prompt

To ensure that the LLM's output is clean and direct, `inkspect` uses a **system prompt** by default. This is a set of instructions that is automatically prepended to every request sent to the LLM. It tells the model to avoid conversational filler like "Of course, here is..." and concluding remarks.

You can see the full system prompt in the configuration file created by the `setup` command.

#### Disabling the System Prompt

In some cases, you might want the LLM to be more conversational. You can disable the system prompt for a single run by using the `--no-system-prompt` flag.

```bash
inkspect optimize --input "Tell me a short story." --no-system-prompt
```

### Prompt Styles (`--style`)

This is where the power of `inkspect` comes in. Use the `--style` flag with the `optimize` command to transform your input in different ways.

#### `refine` (Default)

Improves the clarity, specificity, and overall quality of your text.

*   **Example:**
    ```bash
    inkspect optimize --style refine --input "what is rust?"
    ```
*   **Sample Output:**
    > Rust is a modern systems programming language focused on performance, reliability, and productivity. It achieves the raw speed of C++ but with powerful compile-time guarantees that prevent common memory bugs.

#### `simplify`

Makes complex topics easier to understand.

*   **Example:**
    ```bash
    inkspect optimize --style simplify --input "Explain quantum computing in simple terms."
    ```
*   **Sample Output:**
    > Imagine a regular computer bit is a light switch that can be either on (1) or off (0). A quantum bit (qubit) is like a dimmer switch; it can be on, off, or somewhere in between, all at the same time. This "in-between" state lets quantum computers explore many possibilities at once, making them incredibly powerful for solving certain types of complex problems.

#### `boost_engagement`

Rewrites text to be more engaging and suitable for social media.

*   **Example:**
    ```bash
    inkspect optimize --style boost_engagement --input "Our new app is now available for download."
    ```
*   **Sample Output:**
    > 🔥 BIG NEWS! 🔥 Our brand new app has officially dropped! 🚀 Download it now and experience the future. You don't want to miss this! #NewApp #LaunchDay #Tech

#### `code-agent-spec`

Transforms a high-level feature request into a detailed, TDD-focused specification for an AI coding agent. **This does not write code, just specs.**

*   **Example:**
    ```bash
    inkspect optimize --style code-agent-spec --input "I want to create a simple command-line todo list app in Rust"
    ```
*   **Sample Output:**
    > ### 1. High-Level Goal
    > Create a command-line interface (CLI) application in Rust for managing a todo list.
    >
    > ### 2. Key Features
    > - Add a new task.
    > - List all tasks.
    > - Mark a task as complete.
    > - Persist tasks to a local file.
    >
    > ### 3. Proposed Architecture & File Structure
    > A single `main.rs` file will contain all the logic. Tasks will be stored in a `todos.json` file.
    >
    > ### 4. Step-by-Step TDD Implementation Plan
    > **Feature: Add a Task**
    > - **Test:** Write a failing test `test_add_task` that checks if a new task is added to the list.
    > - **Implementation:** Create the `add_task` function and the `Task` struct.
    > ...

#### `code-gen`

Generates a complete, single-file application from a high-level description.

*   **Example:**
    ```bash
    inkspect optimize --style code-gen --input "create a simple command-line todo list app in Rust"
    ```
*   **Sample Output:**
    > ```rust
    > use clap::{Parser, Subcommand};
    > use serde::{Deserialize, Serialize};
    > use std::fs;
    > // ... (complete, working Rust code for a todo app) ...
    > ```
    >
    > ### How to Build and Run
    > 1.  **Save the code** to `src/main.rs`.
    > 2.  **Add dependencies** to `Cargo.toml`: `clap`, `serde`, `serde_json`.
    > 3.  **Run** `cargo build --release`.
    > 4.  **Execute** `./target/release/todo-cli add "My first task"`.
