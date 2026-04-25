import { createInterface } from "node:readline/promises";

export async function yes_no(prompt: string) {
  const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  const ans = await rl.question(`${prompt} (y/n): `);
  rl.close();

  return ans.toLowerCase() === "y";
}

export function fmt_output(s: string) {
  const l = s.length;
  const padding = 2;
  const bar = "─".repeat(l + padding);
  const top_bar = "╭" + bar + "╮";
  const bottom_bar = "╰" + bar + "╯";

  const boddys = s.split("\n").map(line => `│ ${line} │`).join("\n");

  return `${top_bar}\n${boddys}\n${bottom_bar}`;
}

