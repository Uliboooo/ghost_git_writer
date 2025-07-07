# change log

## 0.1.1 by jul-7 16:30

Based on the diff, here is a summary of the changes:

  This set of changes refactors the way LLM providers are configured and called.


   - Dependency Updates:
     - The derive-getters crate has been added to Cargo.toml.


   - Configuration (`src/config.rs`):
     - The configuration for LLM providers has been centralized into a ModelInfo struct, which is now equipped with getters.
     - A new Provider enum has been introduced to represent the different supported LLM services (Ollama, Anthropic, Deepseek, Gemini, OpenAI).
     - A CustomPrompt struct has been added to the configuration.


   - LLM Logic (`src/llm.rs`):
     - The call_llms function signature has been simplified. It now accepts the ModelInfo struct instead of numerous individual parameters.
     - The function now uses the Provider enum to determine which LLM service to call.


   - Main (`src/main.rs`):
     - A new prompt module has been added.



