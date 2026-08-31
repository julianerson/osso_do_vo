use axum::{Json, Router, http::header::CONTENT_TYPE, routing::post};
use serde::{Deserialize, Serialize}; // Adicionado o Serialize para responder em JSON
use std::io::{self, Write};
use tower_http::cors::{CorsLayer, Any};

use std::{cmp::Ordering,thread, time::Duration};
mod funcoes;//pega o funcoes.rs, tambem podemos colocar um mod dentro do codigo
use std::fs::OpenOptions;
pub trait PERDEDOR  {
    fn perd (&self)->String;
}
pub struct PARTIDA{//struct que guardam os dados
    nome: String,
    resultado: String,
    dificuldade: String,
    pontuacao: i32,
}
impl PERDEDOR for PARTIDA {
    fn perd (&self)->String {
        format!("{} is a loser in according to https://theonion.com/ the americas ~~funiest~~ finest news source",self.nome)
    }
}
#[derive(Deserialize)]
struct Usuario {
    numero:u128,
    requis:u32,
    querer:String,
    nome: String,
    difi: String,
    planilha:bool,
}

// Criamos uma estrutura para a resposta do servidor
#[derive(Serialize)]
struct Resposta {
    mensagem: String,
    status: String,
    numerr:u128,
    atim:i128,
}

#[tokio::main]
async fn main() {
    print!("[SERVIDOR] Inicializando agora...\n");
    let _ = io::stdout().flush();
    let cors = CorsLayer::new()
    .allow_origin(Any) // Em produção, mude para a URL exata do seu frontend
    .allow_methods(Any)
    .allow_headers([CONTENT_TYPE]);
    // Mantive a rota na raiz "/" igual você configurou no seu teste
    let app = Router::new().route("/", post(criar_usuario)).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = io::stdout().flush();

    axum::serve(listener, app).await.unwrap();
}

// Alteramos o retorno de `String` para `Json<Resposta>`
async fn criar_usuario(Json(payload): Json<Usuario>) -> Json<Resposta> {
    let _ = io::stdout().flush();
    let mes:String;
    let nom:u128;
    let stai:String;
    let lerygo:i128;
    if payload.querer == "jogo"{
        mes = format!("Usuário {} lido com sucesso no Axum!", payload.nome);
        stai = "sucesso".to_string();
        nom = match payload.difi.as_str() {
            "FACIL" => 20,
            "MEDIO" =>15,
            "DIFICIL"=>10,
            _=>100,
        };
        lerygo = rand::random_range(1..=1000);
    }else if payload.querer == "jogando"{
        mes = format!("Usuário {} lido com sucesso no Axum!", payload.nome);
        stai = "sucesso".to_string();
        nom = match payload.difi.as_str() {
            "FACIL" => 20,
            "MEDIO" =>15,
            "DIFICIL"=>10,
            _=>100,
        };
        lerygo = 0
    }else {
        mes = "Erro ao processar".to_string();
        stai = "falha".to_string();
        nom = 0;
        lerygo = 0
    }
    let resposta = Resposta {
        mensagem: mes,
        status: stai,
        numerr: nom,
        atim: lerygo
    };

    // Retorna o JSON estruturado para o cliente
    Json(resposta)
}
