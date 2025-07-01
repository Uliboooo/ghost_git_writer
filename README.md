# ghost writer

## now, supported llms

- [x] ollama
- [ ] openAI
- [ ] gemini
- [ ] claude

## default operation

```bash
ggw cmt
-> msg: feat(cli): add `cmt` subcommand for commit message generatcmt   <---┐
-> cloud you commit by this message?(y/n)                                   |
                                                                            |
# if you chose `y`                                                          |
commited by feat(cli): add `cmt` subcommand for commit message generation   |
                                                                            |
# if you chose `n`                                                          |
-> do you ask to rewrite?(msg/n)                                            |
-> [your rewrite prompt]                                                    |
# loop ---->----------------->------------------->--------------------->----┘
```

## subcommand

| subcommand | desc |
| :---: | :---: |
| `cmt` | commit message |
| `rdm` | write a readme |

## options

- `-y --yes`: don't confirm run git command
- `-r --norewrite`: don't ask if rewrite.
- `-s --servie<service-name>`: select service
- `-m --model <model name>`: select model. **require `-s`**

`cmt` options

- `-c --no-commit`: don't `git add & commit, only show`

## usage

```bash
# create commit msg. then output created msg to stdout.
ggw cmt
-> xxx

# create commit msg and automacitty `git add. && commit`
ggw cmt -y
```

## will features

- [ ] rewrite features
- [ ] 

