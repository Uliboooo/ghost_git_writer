#!/usr/bin/env bun

import { Command } from "commander";
import simpleGit from "simple-git";
import { callLLM } from "packages/core/src/llm";
import { spinner } from "packages/core/src/cui/spinner";
import { fmt_output, yes_no } from "packages/core/src/cui/prompt";
import { model_name_resolver } from "packages/core/src/cli/parser";

const program = new Command();

program.name("ggw").description("Ghost git Writer - CLI tool for AI-powered commits").version("0.1.0");

program
  .option("-m, --model <type>", "LLM model to use (gemini)", "gemini/gemini-3-flash-preview")
  .option("-c, --config <path>", "path to config file")
  .option("-l, --lang <lang>", "select lang");

program.parse();

const options = program.opts();
const lang = String(options.lang);
const [pro, model] = model_name_resolver(options.model);

const git = simpleGit();
const diff = await git.diff();

const prompt = `You are an assistant that writes Git commit messages.\
When code changes include modifications to documentation files (e.g., README.md, docs/), ignore those changes and generate the commit message based solely on source code changes.\
Given a description of code changes, output only a single-line commit message in Conventional Commits format (e.g., \"feat:\", \"fix:\", \"docs:\", etc.).\
Do not include any extra text, code blocks, or formatting. Only output the commit message.\
git status info and diff changes:\
${diff}
Please answer in ${lang}`

const cmt_msg_p = callLLM(pro, model, prompt);

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

