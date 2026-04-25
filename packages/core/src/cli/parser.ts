export type ModelInfo = [provider: string, model_name: string];

export function model_name_resolver(m: string): ModelInfo {
  const input_model = ((inp => {
    const cut = String(inp).split("/");
    if (cut.length >= 2) {
      return [String(cut[0]), String(cut[1])]
    }
  })(m));

  const pro = (input_model?.[0] ?? "gemini").toLowerCase();
  const model = input_model?.[1] ?? "gemini-3-flash-preview";
  return [pro, model];
}
