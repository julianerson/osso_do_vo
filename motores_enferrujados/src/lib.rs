
use wasm_bindgen::prelude::*;//importa tudo na prelude que e usado no codigo
use wasm_bindgen::JsCast;//importa as funçoes responsaveis por transoformar os elements
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlFormElement, HtmlInputElement,HtmlSelectElement, SubmitEvent,console};//pega as apis do navegados
use reqwest::Client;
use serde::{Deserialize, Serialize}; // Adicionado o Serialize para responder em JSON
use std::rc::Rc;
use std::cell::RefCell;

#[wasm_bindgen]
pub fn test() {
    let window = web_sys::window().expect("Sem janela");
    let document = window.document().expect("Sem documento");

    // Estado compartilhado, criado uma única vez
    let respondido: Rc<RefCell<Resposta>> = Rc::new(RefCell::new(Resposta {
        mensagem: "".to_string(),
        status: "".to_string(),
        numerr: 0,
    }));

    if let Some(form_element) = document.get_element_by_id("meu-formulario") {
        if let Ok(form) = form_element.dyn_into::<HtmlFormElement>() {

            let respondido_clone = respondido.clone(); // clona pra mover pra dentro da closure

            let callback = Closure::<dyn FnMut(SubmitEvent)>::new(move |evento: SubmitEvent| {
                evento.prevent_default();
                let hostname = web_sys::window().expect("sem hostname").location().hostname().unwrap();
                let doc = web_sys::window().unwrap().document().unwrap();
                let dificuldade = doc.get_element_by_id("dificuldade").unwrap().dyn_into::<HtmlSelectElement>().expect("oi").value();

                let mut nome_capturado = String::new();
                if let Some(input) = doc.get_element_by_id("nome") {
                    if let Ok(input_html) = input.dyn_into::<HtmlInputElement>() {
                        nome_capturado = input_html.value();
                        input_html.set_value("");
                    }
                }

                let respondido_async = respondido_clone.clone(); // clona de novo pro async move
                let doc_async = doc.clone();
                spawn_local(async move {
                    console::log_1(&JsValue::from_str(&format!("{} Iniciando a requisição...", &hostname)));

                    match envio(nome_capturado, dificuldade, hostname).await {
                        Ok(texto_resposta) => {
                            console::log_1(&JsValue::from_str(&format!("Resposta do Servidor: {}", texto_resposta.status)));
                            *respondido_async.borrow_mut() = texto_resposta; // grava na caixinha compartilhada
                        }
                        Err(erro) => {
                            console::log_1(&JsValue::from_str(&format!("Erro ao conectar: {:?}", erro)));
                        }
                    }
                    // O query_selector_all aceita seletores CSS, então usamos ".nome-da-classe"
                    if let Ok(lista_elementos) = doc_async.query_selector_all(".perguntas") {
                        for i in (0..lista_elementos.length()).rev() {
                            if let Some(nodo) = lista_elementos.get(i) {
                                // Em web-sys, precisamos converter o Node em Element para poder usar o .remove()
                                if let Ok(elemento) = nodo.dyn_into::<web_sys::Element>() {
                                    elemento.remove();
                                }
                            }
                        }
                    }
                    if let Some(elemento_generico) = doc_async.get_element_by_id("meu-formulario") {
                        // 1. Convertemos o formulário para HtmlFormElement (ou HtmlElement) para anexar filhos
                        if let Ok(formulario) = elemento_generico.dyn_into::<web_sys::HtmlFormElement>() {
                            
                            // 2. Criamos o elemento de INPUT que você quer colocar na tela
                            if let Ok(input_generico) = doc_async.create_element("input") {
                                if let Ok(inp) = input_generico.dyn_into::<web_sys::HtmlInputElement>() {
                                    
                                    // 3. Configura as propriedades do input
                                    inp.set_type("number");
                                    inp.set_id("valoresss");
                                    inp.set_placeholder("digite os chutes");

                                    // 4. Anexa o INPUT dentro do FORMULÁRIO
                                    let _ = formulario.append_child(&inp);
                                    let ju =&respondido_async.borrow().numerr;
                                    for i in 0..*ju{
                                        
                                    }
                                }
                            }
                        }
                    }
                    
                    
                });
                
            });

            let _ = form.add_event_listener_with_callback("submit", callback.as_ref().unchecked_ref());
            callback.forget();
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
