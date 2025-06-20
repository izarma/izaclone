// region: -- Modules

use izaclone::{Result, DEFAULT_SYSTEM_MOCK, MODEL};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

// endregion: -- Modules

#[tokio::main]
async fn main() -> Result<()>{
    // By default localhost:11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompt = "What is the best programming language? (Be concise)".to_string();
    let gen_req = GenerationRequest::new(model, prompt)
		.system(DEFAULT_SYSTEM_MOCK.to_string());

	// -- Stream Response Generation
	let res = ollama.generate(gen_req).await?;
    println!("->> res {}", res.response);
    Ok(())
}