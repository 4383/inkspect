You are a senior Rust developer and expert in CLI tools, modular architecture, and LLM API integration.
You are following a strict **Test-Driven Development (TDD)** methodology. For every module or functionality, follow this loop:

1. Write failing **unit tests** first that specify the desired behavior.
2. Run the tests and observe failures.
3. Implement only the code necessary to make the tests pass.
4. Refactor if needed and re-run tests.

---

### 🎯 Project: `inkspect`

`inkspect` is a CLI tool to help users **edit, refine, and optimize natural language prompts** using Language Model APIs. The user can write prompts in their favorite editor, and the tool sends them to an LLM (e.g., Gemini or Claude) with an internal instruction for refinement.

---

### 🧩 Core Features

#### Input

* Accepts `--input "..."` or opens an editor (`$EDITOR` or `--editor <editor>`).

#### LLM Backends

* Gemini (default) and Claude supported.
* Selection via `--provider`.
* Prompt instruction (system message) comes from config, selected via `--style`.

#### Output

* Prints refined prompt to stdout or saves it with `--output`.

#### Config

* Loaded from `~/.inkspect.toml` (or `--config`).
* Contains API keys, prompt styles, and default settings.

---

### 📐 Architecture

* `main.rs`: entry point
* `cli.rs`: CLI parsing (clap)
* `editor.rs`: open and read editor input
* `config.rs`: load and validate config
* `core.rs`: glue logic
* `llm/`:

  * `trait.rs`: `LlmBackend`
  * `gemini.rs`, `claude.rs`: implementations

---

### 🧪 Testing strategy (TDD)

For each module, follow TDD strictly:

* `cli.rs`: test parsing `--input`, `--editor`, `--provider`, `--style`, `--output`
* `config.rs`: test loading config, fallback values, style selection
* `editor.rs`: test launching editors, waiting for them, reading contents
* `llm::trait.rs`: define trait and test with mock implementations
* `llm::gemini.rs`: test interactions using mocked HTTP calls
* `core.rs`: test full user flow using mocks and temporary files

---

### Documentation

This project should be documented in the README file:

* the purpose of this project (what problem it solve and how).
* how to install it
* how to use it
* how to configure it
* usages examples

---

### 🧰 Dependencies

* `clap`, `reqwest`, `serde`, `toml`, `confy` or `config`
* `tempfile`, `std::process::Command`, `which`
* `anyhow` or `thiserror`
* `mockito`, `assert_cmd`, `predicates`

---

### ⚙️ Configuration Example (`~/.inkspect.toml`)

```toml
[llm]
provider = "gemini"
default_prompt = "refine"

[providers.gemini]
api_key = "YOUR_GEMINI_API_KEY"

[providers.claude]
api_key = "YOUR_CLAUDE_API_KEY"

[prompts]
refine = "You are a prompt engineer. Rewrite the input prompt to make it more clear, specific, and LLM-friendly."
simplify = "Make the input prompt more concise and easier to understand."
boost_engagement = "Make the prompt more engaging and suitable for social media."
```

---

### 🧪 Example CLI Use Cases (to be covered in tests)

#### 1. Basic editor-based refinement (default editor and provider)

```bash
inkspect optimize
```

→ Opens `$EDITOR` (e.g., `vim`), user writes prompt → Gemini refines using the default `refine` style → result printed to terminal.

#### 2. Forcing a graphical editor like Gedit

```bash
inkspect optimize --editor gedit
```

→ Opens `gedit`, same flow as above.

#### 3. Inline prompt refinement with explicit style and output

```bash
inkspect optimize --input "give me blog ideas about climate change" \
                  --style boost_engagement \
                  --output refined.txt
```

→ Uses the `boost_engagement` instruction from config, sends prompt to Gemini, saves output in `refined.txt`.

#### 4. Switching backend provider

```bash
inkspect optimize --input "how does a fusion reactor work?" --provider claude
```

→ Sends prompt to Claude using default style.

#### 5. Using a custom config file

```bash
inkspect optimize --config ./custom.toml
```

→ Loads a different config (e.g., different keys or styles).

#### 6. Listing available styles (future feature suggestion)

```bash
inkspect list-styles
```

→ Displays all available prompt styles from config (optional future feature).

---

### 📦 Deliverables

* `Cargo.toml` with all dependencies
* Project folder layout
* **Tests first**, for each module
* Minimal working implementation to pass tests
* Mock-based tests for LLM providers
* A sample `.inkspect.toml` as above
* Idiomatic, modular, and extensible Rust code
* Documented project and usages examples
