/*
wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin
cargo test --features "opencl" -- --nocapture
cargo test --release --features "opencl" -- --nocapture
*/
use crate::{config::TranscribeOptions, transcribe::create_context};
use serial_test::serial;
use std::path::PathBuf;
use tracing_test::traced_test;
use std::time::Instant;


#[test]
#[serial]
#[traced_test]
fn test_transcribe() {
    let ctx = create_context(&PathBuf::from("../ggml-medium.bin"), None).unwrap();
    let options = &TranscribeOptions {
        init_prompt: None,
        lang: Some("en".into()),
        max_sentence_len: None,
        path: "../samples/single.wav".into(),
        verbose: None,
        max_text_ctx: None,
        n_threads: None,
        temperature: None,
        translate: None,
        word_timestamps: None,
    };
    let start = Instant::now();
    let result = crate::transcribe::transcribe(&ctx, options, None, None, None, None);
    println!("{:?}", result);
    println!("Elapsed time: {:.2} seconds", Instant::now().duration_since(start).as_secs_f64());
}
