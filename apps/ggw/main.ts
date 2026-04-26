#!/usr/bin/env bun

import { Command } from "commander";
import simpleGit from "simple-git";
import { callLLM } from "packages/core/src/llm";
import { spinner } from "packages/core/src/cui/spinner";
import { fmt_output, yes_no } from "packages/core/src/cui/prompt";
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
  .option("-c, --config <Path>", "path to config file")
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

const prompt = `You are an assistant that writes Git commit messages.\
When code changes include modifications to documentation files (e.g., README.md, docs/), ignore those changes and generate the commit message based solely on source code changes.\
Given a description of code changes, output only a single-line commit message in Conventional Commits format (e.g., \"feat:\", \"fix:\", \"docs:\", etc.).\
Do not include any extra text, code blocks, or formatting. Only output the commit message.\
git status info and diff changes:\
${diff}
Please answer in ${lang}`

const cmt_msg_p = callLLM(provider, model, prompt);

const cmt_msg = await spinner(cmt_msg_p, `Calling ${model} ...`);

console.log(fmt_output(cmt_msg));

const git_res = await (async (y_n: boolean) => {
  if (y_n) {
    const res_git_add = await git.add(".");
    const res_git_cmt = await git.commit(cmt_msg);
    return [res_git_add, res_git_cmt];
  } else {
    console.error("cannceld.");
    process.exit(1);
  }
})(await yes_no("git commit as this message?"));

if (git_res == null) {
  console.log("failed to git add or commit");
} else {
  console.log("ok.");
}

