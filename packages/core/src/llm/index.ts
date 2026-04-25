import { call_gemini } from "./gemini";
import { call_openAI } from "./openai";

export function callLLM(provider: string, model: string, prompt: string) {
  switch (provider.toLowerCase()) {
    case "gemini":
      return call_gemini(model, prompt);
    case "openai":
      return call_openAI(model, prompt);
    default:
      throw new Error("unknown provider");
  }
}
