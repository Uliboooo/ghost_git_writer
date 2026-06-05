import { beforeEach, describe, expect, test, mock } from "bun:test";
import { call_openAI } from "./openai";

const mockCreate = mock(() => {
  return (async function* () {
    yield { choices: [{ delta: { content: "Hello" } }] };
    yield { choices: [{ delta: { content: " world" } }] };
  })();
});

mock.module("openai", () => {
  return {
    default: class {
      chat = {
        completions: {
          create: mockCreate,
        },
      };
    },
  };
});

describe("call_openAI", () => {
  beforeEach(() => {
    delete process.env["OPENAI_API_KEY"];
    mockCreate.mockClear();
  });

  test("should throw if api key is missing", async () => {
    await expect(call_openAI("model", "prompt")).rejects.toThrow("OPENAI_API_KEY is not set");
  });

  test("should call openai and return combined response", async () => {
    process.env["OPENAI_API_KEY"] = "fake-key";
    const result = await call_openAI("test-model", "test-prompt");
    
    expect(result).toBe("Hello world");
    expect(mockCreate).toHaveBeenCalledWith({
      messages: [{ role: 'user', content: "test-prompt" }],
      model: "test-model",
      stream: true,
    });
  });
});
