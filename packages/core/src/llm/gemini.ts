import { GoogleGenAI } from "@google/genai";

export async function call_gemini(model: string, prompt: string) {
  const ai = new GoogleGenAI({});
  const res = await ai.models.generateContent({
    model: model,
    contents: prompt
  });

  return res.text ?? "nothing";
}
