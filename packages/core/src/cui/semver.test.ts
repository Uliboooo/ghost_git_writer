import { expect, test, describe } from "bun:test";
import { parseSemVerPart, semVerSelector } from "./semver";

describe("semver", () => {
  describe("parseSemVerPart", () => {
    test("should parse valid parts", () => {
      expect(parseSemVerPart("major")).toBe("major");
      expect(parseSemVerPart("minor")).toBe("minor");
      expect(parseSemVerPart("patch")).toBe("patch");
    });

    test("should handle case-insensitivity and whitespace", () => {
      expect(parseSemVerPart(" MAJOR ")).toBe("major");
      expect(parseSemVerPart("Minor")).toBe("minor");
    });

    test("should return null for invalid parts", () => {
      expect(parseSemVerPart("invalid")).toBeNull();
      expect(parseSemVerPart("")).toBeNull();
    });
  });

  describe("semVerSelector", () => {
    test("should generate correct selector for major", () => {
      const output = semVerSelector("major");
      expect(output).toContain("╭───────╮");
      expect(output).toContain("│ MAJOR │");
      expect(output).toContain("╰───────╯");
    });

    test("should generate correct selector for minor", () => {
      const output = semVerSelector("minor");
      expect(output).toContain("╭───────╮");
      expect(output).toContain("│ MINOR │");
      expect(output).toContain("╰───────╯");
    });

    test("should generate correct selector for patch", () => {
      const output = semVerSelector("patch");
      expect(output).toContain("╭───────╮");
      expect(output).toContain("│ PATCH │");
      expect(output).toContain("╰───────╯");
    });
  });
});
