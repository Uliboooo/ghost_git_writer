import { expect, test, describe } from "bun:test";
import { buildSplitPrompt, parseSplitResponse } from "./index";

describe("split", () => {
  describe("buildSplitPrompt", () => {
    test("includes the changed files, status, diff and language", () => {
      const prompt = buildSplitPrompt(
        '{"modified":["a.ts"]}',
        "diff --git a/a.ts b/a.ts",
        ["a.ts", "b.ts"],
        "Japanese",
      );
      expect(prompt).toContain("- a.ts");
      expect(prompt).toContain("- b.ts");
      expect(prompt).toContain('{"modified":["a.ts"]}');
      expect(prompt).toContain("diff --git a/a.ts b/a.ts");
      expect(prompt).toContain("Japanese");
    });
  });

  describe("parseSplitResponse", () => {
    const known = ["src/a.ts", "src/b.ts", "README.md"];

    test("parses plain JSON into groups", () => {
      const raw = JSON.stringify({
        groups: [
          { message: "feat: add a", files: ["src/a.ts"] },
          { message: "docs: update readme", files: ["README.md"] },
        ],
      });
      const groups = parseSplitResponse(raw, known);
      expect(groups).toEqual([
        { message: "feat: add a", files: ["src/a.ts"] },
        { message: "docs: update readme", files: ["README.md"] },
        { message: "chore: apply remaining changes", files: ["src/b.ts"] },
      ]);
    });

    test("tolerates markdown code fences and surrounding prose", () => {
      const raw =
        'Here is the split:\n```json\n{"groups":[{"message":"fix: b","files":["src/b.ts"]}]}\n```\nDone.';
      const groups = parseSplitResponse(raw, ["src/b.ts"]);
      expect(groups).toEqual([{ message: "fix: b", files: ["src/b.ts"] }]);
    });

    test("drops unknown paths", () => {
      const raw = JSON.stringify({
        groups: [
          { message: "feat: a", files: ["src/a.ts", "does/not/exist.ts"] },
        ],
      });
      const groups = parseSplitResponse(raw, ["src/a.ts"]);
      expect(groups).toEqual([{ message: "feat: a", files: ["src/a.ts"] }]);
    });

    test("assigns each file to only the first group that claims it", () => {
      const raw = JSON.stringify({
        groups: [
          { message: "feat: a", files: ["src/a.ts"] },
          { message: "chore: a again", files: ["src/a.ts"] },
        ],
      });
      const groups = parseSplitResponse(raw, ["src/a.ts"]);
      expect(groups).toEqual([{ message: "feat: a", files: ["src/a.ts"] }]);
    });

    test("collects unassigned files into a fallback group", () => {
      const raw = JSON.stringify({ groups: [] });
      const groups = parseSplitResponse(raw, ["src/a.ts", "src/b.ts"]);
      expect(groups).toEqual([
        {
          message: "chore: apply remaining changes",
          files: ["src/a.ts", "src/b.ts"],
        },
      ]);
    });

    test("skips groups with empty message or no valid files", () => {
      const raw = JSON.stringify({
        groups: [
          { message: "   ", files: ["src/a.ts"] },
          { message: "feat: b", files: [] },
        ],
      });
      const groups = parseSplitResponse(raw, ["src/a.ts", "src/b.ts"]);
      expect(groups).toEqual([
        {
          message: "chore: apply remaining changes",
          files: ["src/a.ts", "src/b.ts"],
        },
      ]);
    });

    test("throws when no JSON object is present", () => {
      expect(() => parseSplitResponse("not json at all", known)).toThrow();
    });
  });
});
