use reqwest::Client;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use mp3_duration;

/// Função para gerar áudio a partir de texto usando a API ElevenLabs.
/// Retorna o caminho do arquivo gerado e sua duração em segundos.
pub async fn generate_audio(message: &str, file_name: &str) -> (String, f64) {
    // URL da API
    let url = "https://api.elevenlabs.io/v1/text-to-speech/EXAVITQu4vr4xnSDxMaL";
    
    // Obtendo a chave da API a partir das variáveis de ambiente
    let api_key = match env::var("ELEVENS_LABS_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("[ERROR] API key não configurada. Verifique as variáveis de ambiente.");
            return ("".to_string(), 0.0);
        }
    };

    // Cliente HTTP
    let client = Client::new();

    println!("[INFO] Enviando texto para ElevenLabs: {}", message);

    // Enviando a requisição
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
                // Lendo o conteúdo do áudio
                match res.bytes().await {
                    Ok(audio_bytes) => {
                        // Salvando o arquivo no sistema
                        let file_path = format!("./voices_marcia/welcome/{}", file_name);
                        let path = Path::new(&file_path);
                        if let Err(e) = File::create(&path).and_then(|mut file| file.write_all(&audio_bytes)) {
                            eprintln!("[ERROR] Falha ao salvar o áudio: {:?}", e);
                            return ("".to_string(), 0.0);
                        }

                        // Calculando a duração do áudio
                        let duration = match mp3_duration::from_path(&path) {
                            Ok(dur) => dur.as_secs_f64(),
                            Err(e) => {
                                eprintln!("[WARNING] Não foi possível determinar a duração do áudio: {:?}", e);
                                0.0
                            }
                        };

                        (file_path, duration)
                    }
                    Err(e) => {
                        eprintln!("[ERROR] Falha ao ler os bytes da resposta da ElevenLabs: {:?}", e);
                        ("".to_string(), 0.0)
                    }
                }
            } else {
                eprintln!("[ERROR] Resposta com falha da ElevenLabs. Status: {}", res.status());
                ("".to_string(), 0.0)
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Falha ao enviar a requisição para ElevenLabs: {:?}", e);
            ("".to_string(), 0.0)
        }
    }
}