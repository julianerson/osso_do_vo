//bem vindo a https://backrooms-wiki.wikidot.com/level-481

use std::{io,cmp::Ordering,thread, time::Duration,io::Write};
mod funcoes;//pega o funcoes.rs, tambem podemos colocar um mod dentro do codigo
use std::fs::OpenOptions;

fn main(){
    let mut pontos = [0,0,0];//aqui sao contados pontos de um jogo, mas pode ser itens em uma compra
    println!("diga seu nome");
    let mut voce = String::new();
    io::stdin().read_line(&mut voce).expect("sla");//le o que vc escreveu
    funcoes::limpar();
    let entrada = funcoes::opções(&["qual vc quer?", "dificil","media","facil"]);
    let dificul: i32 = match entrada {//atribui uma dificuldade, mas e se colocarmos funcionarios?
        "dificil" => {println!("dificul");10},
        "media" => {println!("medio");15},
        "facil" => {println!("facil");20},
        _ => {
            println!("Opção inválida, definindo dificuldade media (15)");
            15
        }};
    let mut jua = false;
    thread::sleep(Duration::from_secs(2));//pausa de 2 seg
    let segrr =[ rand::random_range(1..=1000), rand::random_range(1..=1000), rand::random_range(1..=1000)];//considera por padrao um unsigned de 32 bit
    for (numero,segr) in segrr.iter().enumerate(){ // graças ao &u32 todo segr aqui e uma referencia assim temos que desreferenciar
        funcoes::limpar();
        println!("vamos aprender {dificul}");
        let yo =rand::random_range(322..=1000);
        if *segr < yo{
            println!("e menor que {yo}");
        }else {
            println!("maior que {yo}");
        }
        let foi = adbvi(*segr, dificul);
        jua = foi.0;//tipo aqui
        if jua{
            println!("vc venceu o round {}",&numero+1);
            pontos[numero] = foi.1;
            thread::sleep(Duration::from_secs(2));
        }else {
            println!("vc perdeu");
            thread::sleep(Duration::from_secs(2));
            let mut textin = OpenOptions::new().append(true).create(true).open("perda.txt").expect("erro");
            writeln!(textin,"{voce}").unwrap();
            break;
        }}
    let texto_fim = if !jua{
        "vc perdeu"
    }else{
        "vc ganhou"
    };
    let tet = PARTIDA{//adiciona os dados, exatamente como no banco de dados
        nome: voce.trim().to_string(),
        resultado: texto_fim.to_string(),
        dificuldade: entrada.to_string(),
        pontuacao: ((pontos[0]+pontos[1]+pontos[2])/3) as i32,
    };
    funcoes::salvar_partida(tet).expect("ffudeu");
} 

pub struct PARTIDA{//struct que guardam os dados
    nome: String,
    resultado: String,
    dificuldade: String,
    pontuacao: i32,
}
fn adbvi(segr: u32,dificul:i32)->(bool,i32){
    let mut i: i32=0;
    let mut ju: bool = false;
    while i<dificul {
        
        let mut test: String = String::new(); // let = criar variaveis, por padrao variaveis sao imutaveis, para mutar se usa o mut
        io::stdin() //mesmo que input python scan do C                
            .read_line(&mut test)//le a linha, e o &mut altera de acordo
            .expect("faio in le linha");//tem que ter um aviso de erro toda chamada de string
        i +=1;
        println!("chance: {i}");//{}sao para variaveis
        let test: u32 = match test.trim().parse() {//ve um erro e faz um plano b
                Ok(num) => num,
                Err(_) => {println!("o imbecil isto nao e uma... uqer saber? so por sacanagem vou contar letra por letra");
                for (nun,letra) in (&test.trim().as_bytes()).iter().enumerate(){
                    println!("caractere {} e o numero {}",*letra as char,&nun+1);
                    thread::sleep(Duration::from_millis(500));
                };
                println!("entao e {}",test.trim().len());
                test.trim().len() as u32},
            };//transfor,a string em unsigned 32
        match test.cmp(&segr) {//compara valores, tipo um if e else, assim classificando a data
            Ordering::Less => println!("o n {test} e pequueno"),
            Ordering::Greater => println!("o n {test} e grande! {}", hex::encode("like my ass!")),
            Ordering::Equal => {println!("You win!🎰");
                println!("{}!", hex::encode("foda, e o pix?"));
                ju = true;
                break;}
            }
        }
    println!("a resposta e {segr}");
    (ju,i)
}
