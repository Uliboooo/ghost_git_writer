import { expect, test, describe, beforeEach } from "bun:test";
import { callLLM } from "./index";

describe("llm index", () => {
  beforeEach(() => {
    delete process.env["GEMINI_API_KEY"];
    delete process.env["OPENAI_API_KEY"];
  });

  test("should call gemini when provider is gemini", async () => {
    // We expect it to throw because GEMINI_API_KEY is missing,
    // which proves it called call_gemini.
    await expect(callLLM("gemini", "model", "prompt")).rejects.toThrow("GEMINI_API_KEY is not set");
  });

  test("should call openai when provider is openai", async () => {
    // We expect it to throw because OPENAI_API_KEY is missing,
    // which proves it called call_openAI.
    await expect(callLLM("openai", "model", "prompt")).rejects.toThrow("OPENAI_API_KEY is not set");
  });

  test("should throw error for unknown provider", async () => {
    await expect(callLLM("unknown", "model", "prompt")).rejects.toThrow("unknown provider");
  });
});
