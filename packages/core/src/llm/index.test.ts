import { expect, test, describe, mock, beforeEach } from "bun:test";
import { callLLM } from "./index";

const mockCallGemini = mock(() => Promise.resolve("gemini response"));
const mockCallOpenAI = mock(() => Promise.resolve("openai response"));

mock.module("./gemini", () => ({
  call_gemini: mockCallGemini,
}));

mock.module("./openai", () => ({
  call_openAI: mockCallOpenAI,
}));

describe("llm index", () => {
  beforeEach(() => {
    mockCallGemini.mockClear();
    mockCallOpenAI.mockClear();
  });

  test("should call gemini when provider is gemini", async () => {
    const result = await callLLM("gemini", "model", "prompt");
    expect(result).toBe("gemini response");
    expect(mockCallGemini).toHaveBeenCalledWith("model", "prompt");
  });

  test("should call openai when provider is openai", async () => {
    const result = await callLLM("openai", "model", "prompt");
    expect(result).toBe("openai response");
    expect(mockCallOpenAI).toHaveBeenCalledWith("model", "prompt");
  });

  test("should throw error for unknown provider", () => {
    expect(() => callLLM("unknown", "model", "prompt")).toThrow("unknown provider");
  });
});
