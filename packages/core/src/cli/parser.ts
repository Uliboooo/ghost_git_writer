export type ModelInfo = [provider: string, model_name: string];

export function model_name_resolver(m: string): ModelInfo {
  const cut = String(m).split("/");
  const input_model = cut.length >= 2 ? [cut[0], cut[1]] : undefined;

  const pro = (input_model?.[0] ?? "gemini").toLowerCase();
  const model = input_model?.[1] ?? "gemini-3-flash-preview";
  return [pro, model];
}
