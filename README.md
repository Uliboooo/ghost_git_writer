# ghost writer - `ggw`

[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

**⚠️ beta ⚠️**

<!-- ## Demo

at ver.0.2.1

![gif](./resource/wwg_demo_0_2_1.gif)

https://www.youtube.com/watch?v=6l42h0nn5Sk -->

## usage

### create a git commit msg

```bash
# write a git commit msg from diff.
❯ ggw cmt -m gemini/gemini-2.0-flash
<<<commit mode>>>

read git diff...
creating commmit message...
created msg:docs: Update README with usage examples

do you edit msg?(y/n)n # if chose `y`, you can write a new commit yourself

continue?(y/n)>y
```

## subcommand & options

subcommands

```bash
cmt   gen commit msg and git commit
rdm   create a readme
sum   out diff summary
cst   use custom prompt
help  Print this message or the help of the given subcommand(s)
```

root options

```bash
Options:
  -y, --yes
  -h, --help     Print help
  -V, --version  Print version
```

`cmt` options

```bash
Usage: ggw cmt [OPTIONS]

Options:
  -m, --model <MODEL>  -m gemini/gemini-2.0-flash
  -a, --alias <ALIAS>  registed alias
  -p, --path <PATH>    work path
  -o, --one-line       print only result
  -l, --lang <LANG>    change output language
  -e, --extra <EXTRA>  extra prompt
  -c, --auto-commit    allow auto git commit
  -h, --help           Print help
```

`rdm` options

```bash
Usage: ggw rdm [OPTIONS]

Options:
  -m, --model <MODEL>               -m gemini/gemini-2.0-flash
  -a, --alias <ALIAS>               registed alias
  -p, --path <PATH>                 work path
  -o, --one-line                    print only result
  -l, --lang <LANG>                 change output language
  -e, --extra <EXTRA>               extra prompt
  -s, --sources <SOURCE_PATH_LIST>
  -d, --directory <DIR>
  -m, --allow-merge
  -o, --over-write
  -h, --help                        Print help
```

`sum` options

```bash
Usage: ggw sum [OPTIONS]

Options:
  -m, --model <MODEL>  -m gemini/gemini-2.0-flash
  -a, --alias <ALIAS>  registed alias
  -p, --path <PATH>    work path
  -o, --one-line       print only result
  -l, --lang <LANG>    change output language
  -e, --extra <EXTRA>  extra prompt
  -h, --help           Print help
```

`cst` ottions

```bash
Options:
  -m, --model <MODEL>  -m gemini/gemini-2.0-flash
  -a, --alias <ALIAS>  registed alias
  -p, --path <PATH>    work path
  -o, --one-line       print only result
  -l, --lang <LANG>    change output language
  -e, --extra <EXTRA>  extra prompt
  -h, --help           Print help
```

## result exmaples

```bash
*  <6353330> 2025-07-16 [uliboooo]  (HEAD -> develop) fix: Remove duplicate println and fix typo in prompt
*  <a07f22f> 2025-07-16 [uliboooo]  feat: add auto commit option and yes option
*  <272de26> 2025-07-16 [uliboooo]  docs: Improve commit message generation prompt
```

## supported llm

- [x] ollama
- [x] anthropic
- [x] gemini
- [x] openai
- [x] deepseek

this program get **environment variables** for api_key.
enviroment variable list

| Provider  | API Key Env Variable |
| :-------: | :------------------: |
| anthropic | `GGW_ANTHROPIC_API`  |
|  gemini   |   `GGW_GEMINI_API`   |
|  openai   |   `GGW_OPENAI_API`   |
| deepseek  |  `GGW_DEEPSEEK_API`  |

## Config format (v.0.8.1)

- custom_prompt feature is not yet completed

```json
{
  "prompt": {
    "custom_prompt": {
      "test": "this is test"
    }
  },
  "llm": {
    "default_alias": null,
    "model_alias": {
      "ge": {
        "provider": "gemini",
        "model": "gemini-2.0-flash",
        "temperature": null,
        "max_tokens": null
      }
    }
  }
}
```
