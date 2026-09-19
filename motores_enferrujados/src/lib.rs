
use wasm_bindgen::prelude::*;//importa tudo na prelude que e usado no codigo
use wasm_bindgen::JsCast;//importa as funçoes responsaveis por transoformar os elements
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlFormElement, HtmlInputElement,HtmlSelectElement, SubmitEvent,console};//pega as apis do navegados
use reqwest::Client;
use serde::{Deserialize, Serialize}; // Adicionado o Serialize para responder em JSON
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Mutex, OnceLock};

static NOME: OnceLock<Mutex<String>> = OnceLock::new();

#[wasm_bindgen]
pub async fn test() {
    let window = web_sys::window().expect("Sem janela");
    let document = window.document().expect("Sem documento");

    // Estado compartilhado, criado uma única vez
    let respondido: Rc<RefCell<Resposta>> = Rc::new(RefCell::new(Resposta {
        mensagem: "".to_string(),
        status: "".to_string(),
        numerr: 0,
    }));
    let mut nome_capturado =String::new();
    let hostname = window.location().hostname().unwrap();
    if let Some(input) = document.get_element_by_id("nome") {
        if let Ok(input_html) = input.dyn_into::<HtmlInputElement>() {
            nome_capturado = input_html.value();
            input_html.set_value("");
        }
    }
    let mutex = NOME.get_or_init(|| Mutex::new(String::new()));
    
    // 2. Bloqueia o Mutex para garantir acesso exclusivo
    let mut nome_guardado = mutex.lock().unwrap();
    
    // 3. Modifica o valor interno (use o * na frente)
    *nome_guardado = nome_capturado.clone(); 
    let dificuldade = document.get_element_by_id("dificuldade").unwrap().dyn_into::<HtmlSelectElement>().expect("oi").value();
    match envio(nome_capturado, dificuldade, hostname).await {
        Ok(texto_resposta) => {
            console::log_1(&JsValue::from_str(&format!("Resposta do Servidor: {}", texto_resposta.status)));
            *respondido.borrow_mut() = texto_resposta; // grava na caixinha compartilhada
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
    numerr:u128
}
#[derive(Serialize, Deserialize)]
pub struct Usuario {
    numero:u128,
    requis:u32,
    querer:String,
    nome: String,
    difi: String,
    planilha:bool,
}


pub async fn envio(nome:String,dificuldade: String,hostname:String) -> Result<Resposta, reqwest::Error> {
    let cliente = Client::builder().build()?;

    // 2. Cria os dados que serão enviados
    let novo_usuario = Usuario {
        numero:0,
        requis:0,
        querer:"jogo".to_string(),
        nome: nome,
        difi: dificuldade,
        planilha:false,
    };

    // 3. Faz o POST enviando o objeto como JSON (.json())
    let resposta = cliente.post(format!("http://{}:3000", hostname))
        .json(&novo_usuario)
        .send()
        .await?;

    // 4. Exibe o status e o texto de resposta da API
    let texto = resposta.json::<Resposta>().await?;

    Ok(texto)
}
