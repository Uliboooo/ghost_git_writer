# ghost writer - `ggw`

[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

**⚠️ beta ⚠️** and this doc is unstable to updating now...

## installing

```zsh
cargo install ghost_git_writer
```

## usage

```zsh
ggw <COMMAND> [Options]
```

- Commands
  - `commit`: generate a git commit message from git diff by llm
  - `readme`: generate a README from codebase.
  - `sumdiff`: generate a summry of changes from git diff
- Global Options(mainly)
  - `-m --model`: model sepcific tag. there are tow pattern to specific model.
  - `-p --path`: specific woek path. if it's empty, set current dir path.
  - `-l --lang`: change output language.(default=english). e.g. `-l japanese`
  - `-e --extra`: extra prompt. if you need to append order to llm.
  - `--config`: config file path. if you need to locate other than `~/.config/ggw/config.toml` or `~/.ggw.toml`.

