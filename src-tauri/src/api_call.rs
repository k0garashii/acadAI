use reqwest;
use serde::{Deserialize, Serialize};

// Structure pour la requête à l'API
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

// Structure pour la réponse de l'API
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}
#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>
}
#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}
#[tauri::command]
pub async fn api_call(name:String) -> Result<String, ()> {
    // Remplacez par votre clé API
    let api_key = "AIzaSyDEn5b_CJTInTpIbmMnvHLRb-VOeBuNmOM";
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-exp-1206:generateContent?key={}", api_key);

    // Créez la requête
    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: name,
            }],
        }],
    };

    // Effectuez la requête POST
    let client = reqwest::Client::new();
    let response = client.post(&url)
        .json(&request_body)
        .send()
        .await.expect("Pas de réponse");

    // Gérez la réponse
    if response.status().is_success() {
        let gemini_response: GeminiResponse = response.json().await.expect("Pas de réponse");
        // On accède au premier candidat et on affiche sa réponse.
        if let Some(text) = gemini_response.candidates.first()
                                                .and_then(|c| c.content.parts.first())
                                                .map(|p| &p.text){
            println!("Réponse de Gemini : {}", text);
            Ok(text.clone())
        } else {
            println!("Pas de réponse");
            Err(())
        }
    } else {
        println!("Erreur lors de la requête : {:?}", response.status());
        // Afficher le corps de la réponse pour plus de détails sur l'erreur
        let error_body = response.text().await.expect("Pas de réponse");
        println!("Corps de l'erreur : {}", error_body);
        Err(())
    }
}