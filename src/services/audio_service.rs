use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use uuid::Uuid; 
use mp3_duration;

pub async fn generate_audio(message: &str, file_name: &str) -> String {
    let url = "https://api.elevenlabs.io/v1/text-to-speech/EXAVITQu4vr4xnSDxMaL";
    let api_key = env::var("ELEVENS_LABS_API_KEY").expect("API key not set");

    let client = Client::new();

    println!("[INFO] Enviando texto para ElevenLabs: {}", message);

    let response = client
        .post(url)
        .header("Accept", "audio/mpeg")
        .header("Content-Type", "application/json")
        .header("xi-api-key", api_key)
        .json(&serde_json::json!({
            "text": message,
            "model_id": "eleven_multilingual_v2",
         
            "voice_settings": {
                "stability": 0.7,
                "similarity_boost": 0.8,
                "style": 1.0,
                "use_speaker_boost": true
            },
            "apply_text_normalization": "on",
        }))
        .send()
        .await;
    
    match response {
        Ok(res) => {
            if res.status().is_success() {
                let audio_bytes = match res.bytes().await {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("[ERROR] Falha ao ler os bytes da resposta da ElevenLabs: {:?}", e);
                        return String::new();
                    }
                };
    
                let file_path = format!("./voices_marcia/welcome/{}", file_name);
                let path = Path::new(&file_path);
                let mut file = File::create(&path).expect("[ERROR] Falha ao criar arquivo de áudio");
                file.write_all(&audio_bytes).expect("[ERROR] Falha ao salvar o áudio");

                 let duration = match mp3_duration::from_path(&path) {
                    Ok(dur) => dur.as_secs_f64(),
                    Err(_) => 0.0,
                };
                
                (file_path, duration)
            } else {
                eprintln!("[ERROR] Resposta com falha da ElevenLabs. Status: {}", res.status());
                String::new()
            }
        },
        Err(e) => {
            eprintln!("[ERROR] Falha ao enviar a requisição para ElevenLabs: {:?}", e);
            String::new()
        }
    }
}
