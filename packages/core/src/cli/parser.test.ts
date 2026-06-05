import { expect, test, describe } from "bun:test";
import { model_name_resolver } from "./parser";

describe("parser", () => {
  describe("model_name_resolver", () => {
    test("should resolve model name with provider", () => {
      expect(model_name_resolver("openai/gpt-4")).toEqual(["openai", "gpt-4"]);
      expect(model_name_resolver("gemini/gemini-pro")).toEqual(["gemini", "gemini-pro"]);
    });

    test("should use default provider and model name if not provided", () => {
      expect(model_name_resolver("")).toEqual(["gemini", "gemini-3-flash-preview"]);
    });

    test("should handle case-insensitivity for provider", () => {
      expect(model_name_resolver("OpenAI/gpt-4")).toEqual(["openai", "gpt-4"]);
    });

    test("should handle models without provider", () => {
      expect(model_name_resolver("gpt-4o")).toEqual(["openai", "gpt-4o"]);
      expect(model_name_resolver("custom-model")).toEqual(["gemini", "custom-model"]);
    });

    test("should handle provider only", () => {
      expect(model_name_resolver("openai")).toEqual(["openai", "gpt-4o"]);
      expect(model_name_resolver("gemini")).toEqual(["gemini", "gemini-3-flash-preview"]);
    });
  });
});
