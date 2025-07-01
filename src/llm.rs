use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use tokio::runtime::Runtime;
use crate::ServiceModel;

fn llm(pmt: String, service: ServiceModel) -> Result<String, ()> {
    call_llms(pmt, service)
}

fn call_llms(pmt: String, service: ServiceModel) -> Result<String, ()>{
    match service {
        ServiceModel::Ollama(model) => {
            let rt = Runtime::new().unwrap();
            rt.block_on(ollama(pmt, model))
        },
    }

}

async fn ollama(pmt: String, model: String) -> Result<String, ()> {
    let ollama = Ollama::default();

    let res = ollama.generate(GenerationRequest::new(model, pmt)).await;
    if let Ok(res) = res {
        Ok(res.response.to_string())
    } else {
        Err(())
    }
}
