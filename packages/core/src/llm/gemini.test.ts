import { beforeEach, describe, expect, test } from "bun:test";
import { call_gemini } from "./gemini";

describe("call_gemini", () => {
  beforeEach(() => {
    delete process.env["GEMINI_API_KEY"];
  });

  test("should throw if api key is missing", async () => {
    await expect(call_gemini("model", "prompt")).rejects.toThrow("GEMINI_API_KEY is not set");
  });
});
