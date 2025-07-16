# ghost writer

**⚠️ beta.**

## supported llm

[impl][test]

- [x] [ ] ollama
- [x] [ ] anthropic
- [x] [x] gemini
- [x] [ ] openai
- [x] [ ] deepseek

this program get **environment variables** for api_key.
env var name list

| Provider | API Key Env Variable |
| :---: | :---: |
| anthropic | `GGW_ANTHROPIC_API` |
| gemini | `GGW_GEMINI_API` |
| openai | `GGW_OPENAI_API` |
| deepseek | `GGW_DEEPSEEK_API` |

e.g. set `GGW_GEMINI_API=AAA444KEY` in .env or shell.

## default operation

## subcommand

| subcommand |      desc      |
| :--------: | :------------: |
|   `cmt`    | commit message |

## options

### global options

- `-y --yes`: don't confirm run commands
- `-p --provider [provider name]`: set provider
- `-m --model [model name]`: set use model
- `-m --model [provider/model]`: you can set provider with model in -m option

### `cmt` options

- [ ] `-c --auto-commit"`: auto run git commit without confirm and `-y` option.

## usage

```bash
# write a git commit msg for diff.
ggw -model gemini/gemini-2.0-flash cmt
# ask you that run git commit -m "msg"? by y / n

# auto commit without asking
ggw -model gemini/gemini-2.0-flash cmt -c
# or
ggw -y -model gemini/gemini-2.0-flash cmt
# ⚠️ `-y` flag is used to bypass additional confirmation prompts.)
```

