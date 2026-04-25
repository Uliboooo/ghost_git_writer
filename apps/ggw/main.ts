#!/usr/bin/env bun

import { Command } from "commander";
import simpleGit from "simple-git";
import { GoogleGenAI } from "@google/genai";
import OpenAI from "openai";
import { createInterface } from "node:readline/promises";

function fmt_output(s: string) {
  const l = s.length;
  const padding = 2;
  const bar = "─".repeat(l + padding);
  const top_bar = "╭" + bar + "╮";
  const bottom_bar = "╰" + bar + "╯";

  const boddys = s.split("\n").map(line => `│ ${line} │`).join("\n");

  return `${top_bar}\n${boddys}\n${bottom_bar}`;
}

async function yes_no(prompt: string) {
  const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  const ans = await rl.question(`${prompt} (y/n): `);
  rl.close();

  return ans.toLowerCase() === "y";
}

function spinner<T>(promise: Promise<T>, text = "Processing") {
  const frames = ["-", "\\", "|", "/"];
  return withSpinner(promise, text);

  async function withSpinner<T>(
    promise: Promise<T>,
    text: string
  ): Promise<T> {
    let i = 0;

    process.stdout.write("\x1b[?25l"); // hide cursor

    const interval = setInterval(() => {
      const frame = frames[i = (i + 1) % frames.length];
      process.stdout.write(`\r${frame} ${text}`);
    }, 100);

    try {
      try {
        const result = await promise;
        clearInterval(interval);
        process.stdout.write(`\r✔ Done\n`);
        return result;
      } catch (err) {
        clearInterval(interval);
        process.stdout.write(`\r✖ Error\n`);
        throw err;
      }
    } finally {
      process.stdout.write("\x1b[?25h"); // show cursor
    }
  }

}
async function call_gemini(model: string, prompt: string) {
  const ai = new GoogleGenAI({});
  const res = await ai.models.generateContent({
    model: model,
    contents: prompt
  });

  return res.text ?? "nothing";
}

async function call_openAI(model: string, prompt: string) {
  const client = new OpenAI({
    apiKey: process.env["OPENAI_API_KEY"],
  });

  const chatComp = await client.chat.completions.create({
    messages: [{ role: 'user', content: prompt }],
    model: model,
    stream: true,
  });
  let buf = ""
  for await (const Chunk of chatComp) {
    buf += Chunk;
  }
  return buf;

}

const program = new Command();

program.name("ggw").description("Ghost git Writer - CLI tool for AI-powered commits").version("0.1.0");

program
  .option("-m, --model <type>", "LLM model to use (gemini)", "gemini/gemini-3-flash-preview")
  .option("-c, --config <path>", "path to config file")
  .option("-l, --lang <lang>", "select lang");

program.parse();

const options = program.opts();
const lang = String(options.lang);
const input_model = ((inp => {
  const cut = String(inp).split("/");
  if (cut.length >= 2) {
    return [String(cut[0]), String(cut[1])]
  }
})(options.model));

const pro = (input_model?.[0] ?? "gemini").toLowerCase();
const model = input_model?.[1] ?? "gemini-3-flash-preview";


const git = simpleGit();
const diff = await git.diff();

const prompt = `You are an assistant that writes Git commit messages.\
When code changes include modifications to documentation files (e.g., README.md, docs/), ignore those changes and generate the commit message based solely on source code changes.\
Given a description of code changes, output only a single-line commit message in Conventional Commits format (e.g., \"feat:\", \"fix:\", \"docs:\", etc.).\
Do not include any extra text, code blocks, or formatting. Only output the commit message.\
git status info and diff changes:\
${diff}
Please answer in ${lang}`

const cmt_msg_p = ((input_provider => {
  if (input_provider === "gemini") {
    return call_gemini(model, prompt);
  } else if (input_provider === "openai") {
    return call_openAI(model, prompt);
  } else {
    console.error(`this provider (${input_provider}) was not supported.`)
    process.exit(1);
  }
})(pro));

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

