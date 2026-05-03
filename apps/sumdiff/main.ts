#!/usr/bin/env bun

import { Command } from "commander";
import simpleGit from "simple-git";
import { callLLM } from "packages/core/src/llm";
import { spinner } from "packages/core/src/cui/spinner";
import { model_name_resolver } from "packages/core/src/cli/parser";

type Options = {
  model: string;
  config?: string;
  lang?: string;
  path?: string;
  stdin: boolean;
  diff?: [string, string];
}

const program = new Command();
program.name("ggw").description("Ghost git Writer - CLI tool for AI-powered commits").version("0.1.0");
program
  .option("-m, --model <Provider/Model_Name>", "LLM model to use (gemini)", "gemini/gemini-3-flash-preview")
  // .option("-c, --config <Path>", "path to config file")
  .option("-l, --lang <Lang>", "select lang")
  .option("-p, --path <Path>", "work path. git project root path.")
  .option("-I, --stdin", "use sdtin as diff content")
  .option("-D, --diff <Commit or branch...>", "diff range");
program.parse();

const options = program.opts<Options>();
const lang = options.lang ?? "English";
const [provider, model] = model_name_resolver(options.model);
const git_repo_path = options.path ?? process.cwd();

const git = simpleGit(git_repo_path);

const diff = await (async (use_stdin: boolean) => {
  if (use_stdin) {
    const input = await Bun.stdin.text();
    return input;
  } else {
    return await git.diff(options.diff);
  }
})(options.stdin);

const git_st = JSON.stringify(await git.status());

const prompt = `summarize the git diff changes.
List the key modifications, what was added, removed, or modified, and briefly explain their purpose or impact if possible.
about only changes. must not write about project. you don't readme writer, you summarize diff changes:

status:
${git_st}

diff:
${diff}

Please answer in ${lang}.;
`

const cmt_msg_p = callLLM(provider, model, prompt);

const cmt_msg = await spinner(cmt_msg_p, `Calling ${model} ...`);

console.log(cmt_msg);

