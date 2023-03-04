use std::collections::HashMap;
use std::io;
use serde::{Serialize, Deserialize};
use reqwest::{Client, header, Response};
use serde_json::Value;

// definindo variáveis fixas
const SYSTEM: &str = "Você é um assistente útil";
const MODEL: &str  = "gpt-3.5-turbo";
const TEMPERATURE: f32 = 0.8;
const MAX_TOKENS: i16 = 200;
const API_KEY: &str = "Your API KEY"


#[derive(Serialize, Deserialize)]
struct RequestBody {
    model: String,
    messages: Vec<HashMap<String, String>>,
    temperature: f32,
    max_tokens: i16,
}

fn main() {

    // criando vetor inicial para alimentar o histórico das dúvidas e respostas
    let mut history: Vec<HashMap<String, String>> = vec![];
    let mut system_msg: HashMap<String, String> = HashMap::new();
    system_msg.insert("role".to_string(), "system".to_string());
    system_msg.insert("content".to_string(), SYSTEM.to_string());
    history.push(system_msg);

    loop {
        // chamando a função para printar a dúvida
        let duvida = prompt_user();
        if duvida.trim() == "sair" {
            break;
        }
        let Ok((_resposta_bot, historico)) = chatbot(&duvida,history.clone()) else { todo!() };
        history.extend_from_slice(&historico[&historico.len() - 2..])
    }
}

// função para carregar a pergunta do usuário
fn prompt_user() -> String{
    let mut duvida = String::new();

    println!("\nQual sua dúvida:");
    io::stdin()
        .read_line(&mut duvida)
        .expect("Falha na leitura");
    duvida
}

// função para fazer o request na API do ChatGPT
#[tokio::main]
async fn chatbot(prompt_text: &str, mut history: Vec<HashMap<String, String>>) -> Result<(String,Vec<HashMap<String, String>>), reqwest::Error> {

    // criando o vector com o prompt onde possui a mensagem inicial do sistema, as ultimas duas perguntas do histórico e a pergunta atual
    let prompt = create_prompt(&prompt_text, &history);

    // struct para request
    let request_body = RequestBody {
        model: MODEL.to_string(),
        messages: prompt.clone(),
        temperature: TEMPERATURE,
        max_tokens: MAX_TOKENS
    };

    // fazendo o request na API do ChatGPT
    let response = request(request_body).await?;

    // fazendo o tratamento da resposta do request para que tenhamos apenas a resposta dada pela API do ChatGPT
    let json_string: String = response.text().await.unwrap();
    let json_objeto: Value = json_string.parse().unwrap();
    let resposta = json_objeto["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();

    println!("\nResposta: \n{resposta}");

    // adicionando pergunta e resposta no histórico
    let mut pergunta_atual: HashMap<String, String> = HashMap::new();
    pergunta_atual.insert("role".to_string(), "user".to_string());
    pergunta_atual.insert("content".to_string(), prompt_text.to_string().replace("\r\n", ""));

    let mut resposta_atual: HashMap<String, String> = HashMap::new();
    resposta_atual.insert("role".to_string(), "assistant".to_string());
    resposta_atual.insert("content".to_string(), resposta.to_string());
    history.push(pergunta_atual);
    history.push(resposta_atual);

    Ok((resposta, history))
    }

fn create_prompt(prompt_text: &str, history: &Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    let mut prompt: Vec<HashMap<String, String>> = vec![history[0].clone()]; 
    let prompt_history = if history.len() > 4 {
        history[history.len() - 5..].to_vec()
    } else {
        history.to_vec()
    };

    prompt.extend_from_slice(&prompt_history[1..]);
    let mut user_msg: HashMap<String, String> = HashMap::new();
    user_msg.insert("role".to_string(), "user".to_string());
    user_msg.insert("content".to_string(), prompt_text.to_string());

    prompt.push(user_msg);
    prompt
}

async fn request(request_body: RequestBody) ->  Result<Response, reqwest::Error> {
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static(API_KEY));

    let url = "https://api.openai.com/v1/chat/completions";
    let json_request = serde_json::to_string(&request_body).unwrap();

    let response = client
        .post(url)
        .headers(headers.clone())
        .body(json_request.clone())
        .send()
        .await;

    response
}