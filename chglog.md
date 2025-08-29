# change log

## 0.14.1(Aug 29) (written by gemini 2.5 pro)

### Key Modifications

The most significant change is the introduction of **multi-language support**. A new global command-line option, `-l` or `--lang`, has been added to allow users to specify the output language for LLM-generated content. This feature required extensive refactoring across the application to pass the language parameter to all LLM prompt generation functions.

Accompanying this feature is a major codebase refactoring:
*   The `main` function is now `async`.
*   Modules for generating commits, READMEs, and summaries have been restructured.
*   The LLM calling logic has been centralized and improved to be more modular.
*   Configuration handling was updated to support more advanced options, like custom base URLs for Ollama.
*   The command-line interface (CLI) helpers were improved, replacing a manual spinner with the `indicatif` crate.

### Added

*   **`.github/workflows/rust.yml`**: A `cargo check` step was added to the CI pipeline for faster error checking.
*   **`src/cli_helper.rs`**: A new `Spinner` struct (using the `indicatif` crate) and a `Printer` struct (for formatted box output) were added.
*   **`Cargo.toml`**: Dependencies for `indicatif`, `unicode-width`, and `url` were added to support the new UI and functionality.
*   **`src/main.rs`**:
    *   A global `-l, --lang` CLI option was added to the `RootOptions` struct.
    *   Logic to handle the new `lang` parameter and pass it to LLM functions.
*   **`.gitignore`**: The `src_old/` directory was added to the ignore list.

### Removed

Multiple files were deleted as part of a major code reorganization and cleanup. The functionality from these files was moved into new, refactored modules.

*   **Source Files Deleted:**
    *   `src/cmt_msg.rs`
    *   `src/custom_prompt.rs`
    *   `src/read_codes.rs`
    *   `src/readme.rs`
    *   `src/sum.rs`
    *   `src/storage.rs` (Functionality replaced by the `easy_storage` crate).
*   **Project Files Deleted:**
    *   `a.diff`: A temporary diff file.
    *   `release/*.zip`: Old binary release artifacts.
    *   `resource/wwg_demo_0_2_1.gif`: An old demo GIF.
    *   `test_config.json`, `test_diff.txt`: Old test configuration and data files.
    *   `ulib_owl_release/`: Directory containing old, specific build scripts.
*   **Dependencies Removed:**
    *   The `dialoguer` dependency was removed from `Cargo.toml`.

### Modified

*   **`Cargo.toml` & `Cargo.lock`**: Project version was bumped to `0.14.1`. Dependency versions were updated, with many now using wildcard versions (e.g., `4.5.*`).
*   **`README.md`**: Significantly rewritten to simplify usage instructions, remove outdated information, and document the new `--lang` option.
*   **`src/config.rs`**: Configuration structs were refactored. The `Model` struct now includes a `base_url` field. The logic for resolving models from aliases or defaults has been updated.
*   **`src/get_input.rs`**: Functions now return `Result` instead of panicking on I/O errors, improving robustness.
*   **`src/git.rs`**: The `get_diff` function was enhanced to allow generating a diff between specific commit points, not just against the working directory.
*   **`src/llm.rs`**: Heavily refactored. LLM calls are now managed through a new `LlmReqInfo` struct and a `Provider` enum, making the code more modular and extensible. Spinner logic is now integrated here.
*   **`src/main.rs`**: This file saw the most changes, orchestrating the new multi-language feature and reflecting the overall code restructuring. Error handling was also completely revamped.
*   **`auto_release.bash`**: The script was modified to clean the `release` directory before creating new zip files.

## 0.9.1(Aug 15)

- change priority of config path,
  - primary: `~/.config/ggw/config.toml`
  - secoundary: `~/.ggw.toml`
- improve `sum` prompt
  - suppress summaries other than changes

## 0.9.0

- config format change to toml from json

## 0.8.0

- change additional prompt for `cmt`. `-c --cutom-prompt` -> `-e --extra`.
- remove `-d --default` option

## 0.6.0

- feat: multi lang support

## 0.5.0

- add feat: oneline mode that print only result e.g. generated commit message and summarize diff
- change option format, `ggw -m gemini/foo cmt` -> `ggw cmt -m gemini/foo`

## 0.4.2

- feat: add custom prompt

## 0.4.0

- feat: add feat to spin a spinner during llm processing

## 0.3.6

Here's a breakdown of the changes in the provided Rust code diff:

**Key Modifications:**

- **Argument Handling in `Readme` struct:**
  - The `source_path_list` field in the `Readme` struct was changed from `Vec<String>` to `Option<Vec<String>>`
  - Added logic to handle the mutually exclusive arguments `source_path_list` and `dir`

**Purpose and Impact:**

- **Flexibility in Readme Creation:**

      *   The change in `source_path_list` to be optional enables the user to specify either a list of source files or a directory for generating a README. Previously, only a list of source files was supported.
      *   The `dir` argument allows the program to read all files in a specified directory for README generation.
      *   The `required_unless_present` and `conflicts_with` arguments ensure that either `source_path_list` or `dir` must be provided, but not both. This improves the command-line interface by making the usage cl

  earer and preventing ambiguous configurations.

- **Directory Traversal for Readme Generation:**
  - The code now handles the case where a directory is provided as input for README generation. It reads all files within the directory and uses them as input for the README creation process.
  - The error handling ensures that if neither `source_path_list` nor `dir` is provided, the program will return an error indicating that a file path is not set.

## 0.3.2

### fix

- fix bug; don't require `-s` in rdm subcommand.

## 0.3.0 Jul 17 13:30

- now, abolish `--servie` option. integrated to format: `-m provider/model`
- now, abolish model format: `-s gemini -m gemini-2.0-flash` because 👆
