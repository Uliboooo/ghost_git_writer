import { beforeEach, describe, expect, test } from "bun:test";
import { call_openAI } from "./openai";

describe("call_openAI", () => {
  beforeEach(() => {
    delete process.env["OPENAI_API_KEY"];
  });

  test("should throw if api key is missing", async () => {
    await expect(call_openAI("model", "prompt")).rejects.toThrow("OPENAI_API_KEY is not set");
  });
});
