#!/usr/bin/env bun

import { Command } from "commander";
import simpleGit from "simple-git";
import { callLLM } from "packages/core/src/llm";
import { spinner } from "packages/core/src/cui/spinner";
import { model_name_resolver } from "packages/core/src/cli/parser";
import { parseSemVerPart, semVerSelector } from "packages/core/src/cui/semver";

type Options = {
  model: string;
  lang?: string;
  path?: string;
  stdin: boolean;
  diff?: [string, string];
  oneline: boolean;
}

const program = new Command();
program.name("which-sem").description("Determine which SemVer part (Major/Minor/Patch) should be bumped based on git diff").version("0.1.0");
program
  .option("-m, --model <Provider/Model_Name>", "LLM model to use (gemini)", "gemini/gemini-3-flash-preview")
  .option("-l, --lang <Lang>", "select lang")
  .option("-p, --path <Path>", "work path. git project root path.")
  .option("-I, --stdin", "use stdin as diff content")
  .option("-D, --diff <Commit or branch...>", "diff range")
  .option("-o, --oneline", "output only the SemVer part name");
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

const prompt = `**Output 'SemVer field name' and 'the reason' separated by '|'. About version field name, only contain semver name (Major or Minor or Patch)** Must strictly adhere to this format: 'Minor | Reasons'. In Semantic Versioning, which field version should be incremented? Think with reference to the git diff data:

git status:
${git_st}

diff:
${diff}

Please answer in ${lang}.`

const res_p = callLLM(provider, model, prompt);
const res = await spinner(res_p, `Calling ${model} ...`);

const parts = res.split("|");
const semver_raw = parts[0] ?? "";
const reason = parts[1]?.trim() ?? "";

if (options.oneline) {
  console.log(semver_raw.trim());
} else {
  const part = parseSemVerPart(semver_raw);
  if (part === null) {
    console.log(semver_raw.trim());
  } else {
    const selector = semVerSelector(part);
    console.log(`should increase at\n${selector}`);
    if (reason) {
      console.log(`reasons:\n${reason}`);
    }
  }
}
