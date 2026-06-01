import OpenAI from "openai";

export async function call_openAI(model: string, prompt: string) {
  const client = new OpenAI({
    apiKey: process.env["OPENAI_API_KEY"],
  });

  const chatComp = await client.chat.completions.create({
    messages: [{ role: 'user', content: prompt }],
    model: model,
    stream: true,
  });
  let buf = ""
  for await (const chunk of chatComp) {
    buf += chunk.choices[0]?.delta?.content || "";
  }
  return buf;

}

