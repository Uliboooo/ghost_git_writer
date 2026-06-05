import { beforeEach, describe, expect, test, mock } from "bun:test";
import { call_gemini } from "./gemini";

const mockGenerateContent = mock(() => {
  return Promise.resolve({
    text: "Gemini response"
  });
});

mock.module("@google/genai", () => {
  return {
    GoogleGenAI: class {
      models = {
        generateContent: mockGenerateContent,
      };
    },
  };
});

describe("call_gemini", () => {
  beforeEach(() => {
    delete process.env["GEMINI_API_KEY"];
    mockGenerateContent.mockClear();
  });

  test("should throw if api key is missing", async () => {
    await expect(call_gemini("model", "prompt")).rejects.toThrow("GEMINI_API_KEY is not set");
  });

  test("should call gemini and return text", async () => {
    process.env["GEMINI_API_KEY"] = "fake-key";
    const result = await call_gemini("test-model", "test-prompt");

    expect(result).toBe("Gemini response");
    expect(mockGenerateContent).toHaveBeenCalledWith({
      model: "test-model",
      contents: "test-prompt"
    });
  });

  test("should throw if response does not contain text", async () => {
    process.env["GEMINI_API_KEY"] = "fake-key";
    mockGenerateContent.mockReturnValueOnce(Promise.resolve({}));
    await expect(call_gemini("model", "prompt")).rejects.toThrow("Gemini response does not contain text");
  });
});
