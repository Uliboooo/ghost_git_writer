import { GoogleGenAI } from "@google/genai";

export async function call_gemini(model: string, prompt: string) {
  const apiKey = process.env["GEMINI_API_KEY"];
  if (!apiKey) {
    throw new Error("GEMINI_API_KEY is not set");
  }
  const ai = new GoogleGenAI({ apiKey });
  const res = await ai.models.generateContent({
    model: model,
    contents: prompt
  });

  if (!res.text) {
    throw new Error("Gemini response does not contain text");
  }

  return res.text;
}
