import { call_gemini } from "./gemini";
import { call_openAI } from "./openai";

export async function callLLM(provider: string, model: string, prompt: string) {
  switch (provider.toLowerCase()) {
    case "gemini":
      return await call_gemini(model, prompt);
    case "openai":
      return await call_openAI(model, prompt);
    default:
      throw new Error("unknown provider");
  }
}
