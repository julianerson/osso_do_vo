use axum::{Json, Router, http::header::CONTENT_TYPE, routing::post};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use tower_http::cors::{Any, CorsLayer};

mod funcoes; // pega o funcoes.rs

pub trait PERDEDOR {
    fn perd(&self) -> String;
}

pub struct PARTIDA {
    nome: String,
    resultado: String,
    dificuldade: String,
    pontuacao: i32,
}

impl PERDEDOR for PARTIDA {
    fn perd(&self) -> String {
        format!(
            "{} is a loser in according to https://theonion.com/ the americas ~~funiest~~ finest news source",
            self.nome
        )
    }
}

#[derive(Deserialize)]
struct Usuario {
    numero: u128,
    requis: u32,
    querer: String,
    nome: String,
    difi: String,
    planilha: u128,
}

#[derive(Serialize)]
struct Resposta {
    mensagem: String,
    status: String,
    numerr: u128,
    atim: i128,
}

// Impede que um nome como <script>...</script> vire código na página
fn escapar(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// Páginas como texto comum + .replace(): o format! quebraria com as chaves do JS
fn pagina_jogo(nome: &str) -> String {
    r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="ola.css">
    <title>runbly</title>
</head>
<body>
    <h1 id="titulo">olaaa @@NOME@@</h1>
    <form id="meu-formulario">
        <label class="perguntas" for="resposta">qual o seu palpite?</label>
        <input class="perguntas" type="number" id="resposta" min="0" step="1" required>
        <button type="button" onclick="enviarUsuario()">Enviar</button>
    </form>
    <script>
        async function enviarUsuario() {
            const usuario = {
                numero: Number(document.getElementById("resposta").value),
                requis: 0,
                querer: "jogando",
                nome: "",
                difi: "",
                fim: 0,
            };
            try {
                const resp = await fetch(`http://${location.hostname}:3000`, {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify(usuario),
                });
                const dados = await resp.json();
                if (dados.status === "sucesso") {
                    document.open();
                    document.write(dados.mensagem);
                    document.close();
                }
            } catch (erro) {
                console.error("Erro ao conectar:", erro);
            }
        }
    </script>
</body>
</html>"#.replace("@@NOME@@", &escapar(nome))
}

fn pagina_jogando(nome: &str,dica: &str,chances:u128) -> String {
    r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="ola.css">
    <title>runbly</title>
</head>
<body>
    <h1 id="titulo">olaaa @@NOME@@</h1>
    <form id="meu-formulario">
        <label class="perguntas" for="resposta">##dicass##</label>
        <input class="perguntas" type="number" id="resposta" min="0" step="1" required>
        <button type="button" onclick="enviarUsuario()">Enviar</button>
    </form>
    <script>
        async function enviarUsuario() {
            const usuario = {
                numero: Number(document.getElementById("resposta").value),
                requis: @@CHANCES@@,
                querer: "jogando",
                nome: "",
                difi: "",
                fim: 0,
            };
            try {
                const resp = await fetch(`http://${location.hostname}:3000`, {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify(usuario),
                });
                const dados = await resp.json();
                if (dados.status === "sucesso") {
                    document.open();
                    document.write(dados.mensagem);
                    document.close();
                }
            } catch (erro) {
                console.error("Erro ao conectar:", erro);
            }
        }
    </script>
</body>
</html>"#.replace("@@NOME@@", &escapar(nome)).replace("##dicass##", &escapar(dica)).replace("@@CHANCES@@",&escapar(&chances.to_string()))
}

#[tokio::main]
async fn main() {
    println!("[SERVIDOR] Inicializando agora...");
    let _ = io::stdout().flush();

    let cors = CorsLayer::new()
        .allow_origin(Any) // Em produção, use a URL exata do frontend
        .allow_methods(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new().route("/", post(criar_usuario)).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn criar_usuario(Json(payload): Json<Usuario>) -> Json<Resposta> {
    

    // O if devolve os quatro valores de uma vez: nenhum ramo fica sem atribuir
    let (mes, stai, nom, lerygo): (String, String, u128, i128) =
        if payload.querer == "jogo" {
            let dificuldade: u128 = match payload.difi.as_str() {
                "FACIL" => 20,
                "MEDIO" => 15,
                "DIFICIL" => 10,
                _ => 100,
            };
            println!("sua dificuldade e {}", dificuldade);
            (
                pagina_jogo(&payload.nome),
                "sucesso".to_string(),
                dificuldade,
                rand::random_range(1..=1000),
            )
        } else if payload.querer == "jogando" {
            let dificuldade:u128=(payload.difi.parse().unwrap());
            let mut resoes:String="".to_string();
            if (payload.numero<payload.planilha){
                resoes = format!("sua resposta {} foi muito pequena",payload.numero);
            }

            (
                pagina_jogando(&payload.nome,&resoes,0),
                "sucesso".to_string(),
                dificuldade-1,
                0,
            )
        } else {
            ("Erro ao processar".to_string(), "falha".to_string(), 0, 0)
        };

    Json(Resposta {
        mensagem: mes,
        status: stai,
        numerr: nom,
        atim: lerygo,
    })
}