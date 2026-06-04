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
  const padding = 2;
  const lines = s.split("\n");
  const maxLineLength = lines.reduce((max, line) => Math.max(max, line.length), 0);
  const bar = "─".repeat(maxLineLength + padding);
  const top_bar = "╭" + bar + "╮";
  const bottom_bar = "╰" + bar + "╯";

  const boddys = lines
    .map(line => `│ ${line.padEnd(maxLineLength, " ")} │`)
    .join("\n");

  return `${top_bar}\n${boddys}\n${bottom_bar}`;
}
