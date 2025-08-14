# change log

## 0.9.0

- config format change to toml from json

## 0.8.0

- change additional prompt for `cmt`. `-c --cutom-prompt` -> `-e --extra`.
- remove `-d --default` option

## 0.6.0

- feat: multi lang support

```bash
<<<commit mode>>>

read git diff...
creating commit message...
japanese

⠹created msg:feat: 言語オプションを追加し、LLM呼び出し時に言語を指定できるようにした

do you edit msg?(y/n)n

continue?(y/n)>n
```

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
