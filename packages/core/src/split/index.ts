import { callLLM } from "../llm";

/**
 * A single commit produced by splitting a diff into semantically
 * independent changes. `files` are the paths that should be staged and
 * committed together, `message` is a single-line Conventional Commits
 * message describing them.
 */
export type CommitGroup = {
  message: string;
  files: string[];
};

export type SplitParams = {
  provider: string;
  model: string;
  status: string;
  diff: string;
  files: string[];
  lang: string;
};

/**
 * Build the prompt asking the LLM to group changed files into semantically
 * independent commits.
 */
export function buildSplitPrompt(
  status: string,
  diff: string,
  files: string[],
  lang: string,
): string {
  return `You are an assistant that splits a set of Git changes into multiple, semantically independent commits.
Analyze the git status and diff below and group the changed files so that each group represents ONE logical change.
For each group, write a single-line commit message in Conventional Commits format (e.g. "feat:", "fix:", "docs:", "refactor:", "chore:").
When code changes include modifications to documentation files (e.g. README.md, docs/), base the message on the source code changes.

Rules:
- Assign every changed file to exactly one group. Do not omit any file and do not repeat a file across groups.
- Only use file paths from the "changed files" list below. Do not invent paths.
- Prefer fewer groups; only split when the changes are genuinely unrelated. If everything belongs together, return a single group.
- Output ONLY valid JSON, with no markdown, no code fences and no extra text, matching exactly this schema:
{"groups":[{"message":"<commit message>","files":["<path>"]}]}

changed files:
${files.map((f) => `- ${f}`).join("\n")}

status:
${status}

diff:
${diff}

Write the commit messages in ${lang}.`;
}

/**
 * Extract the outermost JSON object from a raw LLM response, tolerating
 * markdown code fences or surrounding prose.
 */
function extractJson(raw: string): string {
  const trimmed = raw.trim();
  const fenceMatch = trimmed.match(/```(?:json)?\s*([\s\S]*?)```/i);
  const body = fenceMatch ? fenceMatch[1].trim() : trimmed;
  const start = body.indexOf("{");
  const end = body.lastIndexOf("}");
  if (start === -1 || end === -1 || end < start) {
    throw new Error("no JSON object found in LLM response");
  }
  return body.slice(start, end + 1);
}

/**
 * Parse the LLM response into commit groups.
 *
 * The result is validated against `knownFiles`: unknown paths are dropped,
 * each file is assigned to at most one group (first occurrence wins), and
 * any changed file the model failed to assign is collected into a final
 * fallback group so that no change is silently lost.
 */
export function parseSplitResponse(
  raw: string,
  knownFiles: string[],
): CommitGroup[] {
  const parsed = JSON.parse(extractJson(raw)) as unknown;
  const rawGroups =
    parsed && typeof parsed === "object" && Array.isArray((parsed as { groups?: unknown }).groups)
      ? ((parsed as { groups: unknown[] }).groups)
      : [];

  const known = new Set(knownFiles);
  const seen = new Set<string>();
  const groups: CommitGroup[] = [];

  for (const g of rawGroups) {
    if (!g || typeof g !== "object") continue;
    const message = (g as { message?: unknown }).message;
    const rawFiles = (g as { files?: unknown }).files;
    if (typeof message !== "string" || !message.trim()) continue;
    if (!Array.isArray(rawFiles)) continue;

    const files = rawFiles.filter(
      (f): f is string =>
        typeof f === "string" && known.has(f) && !seen.has(f),
    );
    if (files.length === 0) continue;

    files.forEach((f) => seen.add(f));
    groups.push({ message: message.trim(), files });
  }

  const leftover = knownFiles.filter((f) => !seen.has(f));
  if (leftover.length > 0) {
    groups.push({ message: "chore: apply remaining changes", files: leftover });
  }

  return groups;
}

/**
 * Ask the configured LLM to split the given changes into semantically
 * independent commit groups.
 */
export async function splitDiffIntoCommits(
  params: SplitParams,
): Promise<CommitGroup[]> {
  const { provider, model, status, diff, files, lang } = params;
  const prompt = buildSplitPrompt(status, diff, files, lang);
  const raw = await callLLM(provider, model, prompt);
  return parseSplitResponse(raw, files);
}
