use std::io;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

use cocoa_foundation::base::id;
use cocoa_foundation::foundation::NSRunLoop;
use objc::{msg_send, sel, sel_impl};

use tts::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let model = "llama2:latest".to_string();

    let ollama = Ollama::default();

    let prompt = "Describe in five words the Rust programming language.".to_string();

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    // if let Ok(res) = res {
    //     println!("{}", res.response);
    // }

    let mut tts = Tts::default()?;
    let Features { is_speaking, .. } = tts.supported_features();

    tts.speak(res.unwrap().response, false)?;

    let mut _input = String::new();
    // The below is only needed to make the example run on MacOS because there is no NSRunLoop in this context.
    // It shouldn't be needed in an app or game that almost certainly has one already.
    #[cfg(target_os = "macos")]
    {
        let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
        unsafe {
            let _: () = msg_send![run_loop, run];
        }
    }
    // io::stdin().read_line(&mut _input)?;
    Ok(())
}
