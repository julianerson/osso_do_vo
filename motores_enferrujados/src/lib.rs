use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlInputElement, HtmlSelectElement};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

static NOME: OnceLock<Mutex<String>> = OnceLock::new();

#[wasm_bindgen]
pub async fn test() {
    let window = web_sys::window().expect("Sem janela");
    let document = window.document().expect("Sem documento");
    let hostname = window.location().hostname().unwrap();

    // Lê o nome e limpa o campo
    let mut nome_capturado = String::new();
    if let Some(input) = document.get_element_by_id("nome") {
        if let Ok(input_html) = input.dyn_into::<HtmlInputElement>() {
            nome_capturado = input_html.value();
            input_html.set_value("");
        }
    }

    // O bloco {} solta o lock antes do .await
    {
        let mutex = NOME.get_or_init(|| Mutex::new(String::new()));
        *mutex.lock().unwrap() = nome_capturado.clone();
    }

    let dificuldade = document
        .get_element_by_id("dificuldade")
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .expect("dificuldade não é um <select>")
        .value();

    match envio(nome_capturado, dificuldade, hostname).await {
        Ok(resposta) => {
            console::log_1(&JsValue::from_str(&format!("Status: {}", resposta.status)));

            if resposta.status == "sucesso" {
                // O HTML já veio pronto no campo "mensagem"
                document
                    .document_element()
                    .expect("sem <html>")
                    .set_inner_html(&resposta.mensagem);
            }
        }
        Err(erro) => {
            console::log_1(&JsValue::from_str(&format!("Erro ao conectar: {:?}", erro)));
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Resposta {
    mensagem: String,
    status: String,
    numerr: u128,
    atim: i128,
}

#[derive(Serialize, Deserialize)]
pub struct Usuario {
    numero: u128,
    requis: u32,
    querer: String,
    nome: String,
    difi: String,
    planilha: u128,
}

pub async fn envio(
    nome: String,
    dificuldade: String,
    hostname: String,
) -> Result<Resposta, reqwest::Error> {
    let cliente = Client::builder().build()?;

    let novo_usuario = Usuario {
        numero: 0,
        requis: 0,
        querer: "jogo".to_string(),
        nome,
        difi: dificuldade,
        planilha: 0,
    };

    let resposta = cliente
        .post(format!("http://{}:3000", hostname))
        .json(&novo_usuario)
        .send()
        .await?;

    let texto = resposta.json::<Resposta>().await?;

    Ok(texto)
}