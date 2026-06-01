import { expect, test, describe, mock } from "bun:test";
import { fmt_output, yes_no } from "./prompt";

const mockQuestion = mock(() => Promise.resolve("y"));
const mockClose = mock(() => {});

// Mock node:readline/promises
mock.module("node:readline/promises", () => ({
  createInterface: () => ({
    question: mockQuestion,
    close: mockClose,
  }),
}));

describe("prompt", () => {
  describe("fmt_output", () => {
    test("should format output with a box", () => {
      const input = "test message";
      const output = fmt_output(input);
      
      expect(output).toContain("╭──────────────╮");
      expect(output).toContain("│ test message │");
      expect(output).toContain("╰──────────────╯");
    });

    test("should handle multiline messages", () => {
      const input = "line 1\nline 2";
      const output = fmt_output(input);
      
      expect(output).toContain("│ line 1 │");
      expect(output).toContain("│ line 2 │");
    });
  });

  describe("yes_no", () => {
    test("should return true for 'y'", async () => {
      mockQuestion.mockImplementation(() => Promise.resolve("y"));

      const result = await yes_no("Proceed?");
      expect(result).toBe(true);
    });

    test("should return true for 'Y'", async () => {
      mockQuestion.mockImplementation(() => Promise.resolve("Y"));

      const result = await yes_no("Proceed?");
      expect(result).toBe(true);
    });

    test("should return false for 'n'", async () => {
      mockQuestion.mockImplementation(() => Promise.resolve("n"));

      const result = await yes_no("Proceed?");
      expect(result).toBe(false);
    });

    test("should return false for other strings", async () => {
      mockQuestion.mockImplementation(() => Promise.resolve("maybe"));

      const result = await yes_no("Proceed?");
      expect(result).toBe(false);
    });
  });
});
