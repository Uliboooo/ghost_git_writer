# ghost writer - `ggw`

<h1 align="center">
	<img src="https://github.com/Uliboooo/ghost_git_writer/blob/develop/README_resource/log-ggw.png" width="100" alt="Icon"/><br/>
  Ghost git writer
</h1>

[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

**⚠️ beta ⚠️** and this doc is unstable to updating now...

## installing

```zsh
cargo install ghost_git_writer
```

### set api key in enviroment variables

- Gemini
  - `GGW_GEMINI_API`
- Anthropic
  - `GGW_ANTHROPIC_API`
- OpenAI
  - `GGW_OPENAI_API`
- Deepseek
  - `GGW_DEEPSEEK_API`

## usage

```zsh
ggw <COMMAND> [Options]
```

- Sub Commands
  - `commit`: generate a git commit message from git diff by llm
  - `readme`: generate a README from codebase.
  - `sumdiff`: generate a summry of changes from git diff
  - `which-sem`: in Sem Ver, Output which field should be incremented.
- Global Options(mainly)
  - `-m --model`: model sepcific tag. there are tow pattern to specific model.
  - `-p --path`: specific woek path. if it's empty, set current dir path.
  - `-l --lang`: change output language.(default=english). e.g. `-l japanese`
  - `-e --extra`: extra prompt. if you need to append order to llm.
  - `--config`: config file path. if you need to locate other than `~/.config/ggw/config.toml` or `~/.ggw.toml`.
  - `--oneline`: output only llm's return for cli pipes
- Options for `commit`
  - `--auto-commit`: allow auto git commit by generated message
  - `-D --diff <DIFF_COMMIT>`: specify commit hash or tag or git symbolic ref(e.g. 'HEAD')
- Options for `readme`
  - `-s --source <source file list>`: source files path. A list of file paths separated by ','.
  - `-d --directory <source dir>`: souce file folder
  - `--merge-readme`: allow merge to existing README.md
- Options for `sumdiff`
  - `-D --diff <DIFF_COMMIT>`: specify commit hash or tag or git symbolic ref(e.g. 'HEAD')
- Options for `which-sem`
  - `-D --diff <DIFF_COMMIT>`: specify commit hash or tag or git symbolic ref(e.g. 'HEAD')

## Examples

```shell
# give `git diff` to command
ggw commit -m gemini/gemini-2.0-flash

# gice `git diff 76fd1d0` to command
ggw sumdiff -D 76fd1d0 -m gemini/gemini-2.5-pro
```
