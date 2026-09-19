//aqui guardarei funçoes cabulosas
use std::io;
use std::io::Write;
use std::process::Command;
use super::PARTIDA;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rust_xlsxwriter::{Workbook};
use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;

pub fn limpar(){
    #[cfg(target_os = "windows")]
    Command::new("cmd")
    .args(["/C", "cls"])
    .status()
    .expect("Falhou ao limpar a tela");//limpa a sua tela
    #[cfg(target_os = "linux")]
    Command::new("clear")
    .status()
    .expect("Falhou ao limpar a tela");//limpa a sua tela
}

//constante global de pi
pub fn opções<'a>(lista: &'a [&'a str]) -> &'a str {//o <'a> inicia o lifetime e o obriga a referenciar cada vaor ao valor original no main ou neste caso a lista la
    let total = lista.len();
    
    if total == 0 { 
        return ""; 
    } 

    let mut varea = 1; 

    if enable_raw_mode().is_err() { 
        return ""; //se falhao no modo raw oprograma pausa
    }

    loop {
        limpar();
        for i in 0..total {
            
            if i == varea {
                println!("=> {}\r", lista[i]); 
            } else {
                println!("   {}\r", lista[i]);
            }
        }
        io::stdout().flush().unwrap(); //forca a saida das palavras

        if let Ok(Event::Key(key)) = event::read() {//começa a ler o teclado se o raw mode for
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => {
                        if varea > 0 { varea -= 1; }
                        else{ varea = total-1;}
                    }
                    KeyCode::Down => {
                        if varea < total - 1 { varea += 1; 
                        }else {
                            varea = 1;
                        }
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {} 
                }
                if varea == 0{
                    varea = total-1;
                }
            }
        }
    }

    let _ = disable_raw_mode();
    
    io::stdout().flush().unwrap();

    // CORREÇÃO 3: Não precisa criar a variável `mut ret`. 
    // Basta colocar a expressão direta para o Rust retornar o item selecionado.
    lista[varea]
}

pub fn salvar_partida(tet: PARTIDA) -> Result<(), Box<dyn std::error::Error>> {
    let caminho_arquivo = "matriz_exportada.xlsx";
    let nome_aba = "Sheet1";

    // Criamos uma matriz na memória que guardará todas as linhas
    let mut matriz_final: Vec<Vec<String>> = Vec::new();//matriz bidimansional vazia
    

    if Path::new(caminho_arquivo).exists() {//verifica a existencia da planilha
        let mut workbook_leitura: Xlsx<_> = open_workbook(caminho_arquivo)?;//le a planilha e usa o ? para nao ter que usar o match
        if let Ok(range) = workbook_leitura.worksheet_range(nome_aba) {
            for row in range.rows() {
                let linha: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();//transforma tudo em string na linha lida
                matriz_final.push(linha);//adiciona a linha
            }
        }
    } else {
        // Se o arquivo NÃO existia, podemos criar opcionalmente uma linha de cabeçalho
        matriz_final.push(vec![
            "Nome".to_string(),
            "Resultado".to_string(),
            "Dificuldade".to_string(),
            "Pontuação".to_string(),
        ]);
    }

    // 2. Adiciona a nova partida atual no final da matriz
    matriz_final.push(vec![
        tet.nome,
        tet.resultado,
        tet.dificuldade,
        tet.pontuacao.to_string(),
    ]);

    // 3. Reescreve/Salva o arquivo inteiro atualizado usando o 'rust_xlsxwriter'
    let mut workbook_escrita = Workbook::new();
    let worksheet = workbook_escrita.add_worksheet();
    
    // Como agora temos uma matriz bidimensional garantida, usamos o write_row_matrix!
    worksheet.write_row_matrix(0, 0, &matriz_final)?;
    workbook_escrita.save(caminho_arquivo)?;

    Ok(())
}