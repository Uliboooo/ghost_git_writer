export type ModelInfo = [provider: string, model_name: string];

export function model_name_resolver(m: string): ModelInfo {
  if (!m) {
    return ["gemini", "gemini-3-flash-preview"];
  }

  const cut = String(m).split("/");
  if (cut.length >= 2) {
    return [cut[0].toLowerCase(), cut[1]];
  }

  const input = cut[0];
  const low = input.toLowerCase();
  if (low === "openai" || low === "gemini") {
    return [low, low === "openai" ? "gpt-4o" : "gemini-3-flash-preview"];
  }

  // If gpt is in the name, default to openai
  if (low.includes("gpt")) {
    return ["openai", input];
  }

  return ["gemini", input];
}
