use std::io;
struct Tabuleiro {
    matriz: [[char; 3]; 3], 
}
#[derive(Debug, PartialEq)] // Permite imprimir e comparar o enum
enum StatusJogo {
    Vitoria(char), // Armazena quem venceu ('X' ou 'O')
    Empate,
    EmAndamento,
}
fn main() {

    println!("=================== Jogo da Velha =================== ");
    println!("Regras do jogo:");
    println!("O jogador 'X' começa.");
    println!("Escolha uma posição de 1 a 9 conforme o layout:");
    println!(" 1 | 2 | 3 ");
    println!("---+---+---");
    println!(" 4 | 5 | 6 ");
    println!("---+---+---");
    println!(" 7 | 8 | 9 ");
    println!("=====================================================");
    
    let mut tabuleiro = Tabuleiro {
        matriz: [
            ['.', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.'],
        ]
    };
    let mut jogador_atual = 'X';// Jogador 'X' começa
    // ● Laço para jogadas
    loop {
        // Pede a jogada do jogador atual e converte para coordenadas
        let (linha, coluna) = ler_posicao(&tabuleiro, jogador_atual);
        // verifica se a célula está ocupada
        if tabuleiro.matriz[linha][coluna] != '.' {
            println!("\nPosição já ocupada! Tente novamente.");
            continue; // Pula para o início do próximo laço sem trocar de jogador
        }

        // Atualiza o tabuleiro com a jogada
        tabuleiro.matriz[linha][coluna] = jogador_atual;

        // Verifica se a partida terminou
        let status = verificar_fim_da_partida(&tabuleiro, linha, coluna);
        
        // Usa um 'match' para tratar os diferentes status do jogo
        match status {
            //Se um jogador ganhar, finalizar a função (com break)
            StatusJogo::Vitoria(vencedor) => {
                println!("\n--- FIM DE JOGO ---");
                mostrar_tabuleiro(&tabuleiro);
                println!("O jogador '{}' venceu!", vencedor);
                break; 
            },
            //Detectar se houve empate
            StatusJogo::Empate => {
                println!("\n--- FIM DE JOGO ---");
                mostrar_tabuleiro(&tabuleiro);
                println!("O jogo terminou em empate!");
                break; 
            },
            // Se o jogo continua, passa para o próximo jogador
            StatusJogo::EmAndamento => {
                jogador_atual = if jogador_atual == 'X' { 'O' } else { 'X' };
            }
        }
    }
}

fn mostrar_tabuleiro(tabuleiro: &Tabuleiro) {
    for (i, linha) in tabuleiro.matriz.iter().enumerate() {
        //i: recebe o índice de linha; linha: recebe a linha atual(vetor[char;3])
        println!(" {} | {} | {} ", linha[0], linha[1], linha[2]
        );
        if i < 2 {
            println!("---+---+---");
        }
    }
    println!();
}

fn converter_indice_para_coordenada(indice: usize) -> Option<(usize, usize)>{
    let indice_base_zero = indice - 1; //convertendo pra base 0 a 8
    let linha = indice_base_zero/ 3;
    let coluna = indice_base_zero % 3;
    Some((linha, coluna))
}

fn ler_posicao(tabuleiro: &Tabuleiro, jogador: char) -> (usize, usize) {
    loop {
        mostrar_tabuleiro(tabuleiro);
        println!("Jogador '{}', escolha a sua jogada (1-9):", jogador);

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erro de leitura. Tente novamente");
            continue;
        }

        let indice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nEntrada inválida. Digite um número de 1 a 9.");
                continue;
            }
        };

        match converter_indice_para_coordenada(indice) {
            Some(coordenada) => {
                return coordenada; // Retorna a coordenada se a conversão for bem-sucedida
            },
            None => {
                println!("\nNúmero fora do intervalo. Escolha um número entre 1 e 9.");
                // O loop continua
            }
        }
    }
}

fn verificar_fim_da_partida(tabuleiro: &Tabuleiro, linha: usize, coluna: usize) -> StatusJogo {
    // caractere do jogador que acabou de fazer a jogada
    let jogador = tabuleiro.matriz[linha][coluna];

    // Se o espaço estiver vazio ('.'), algo está errado, mas o jogo continua
    if jogador == '.' {
        return StatusJogo::EmAndamento;
    }

    // Verifica a linha da jogada
    if tabuleiro.matriz[linha][0] == jogador &&
       tabuleiro.matriz[linha][1] == jogador &&
       tabuleiro.matriz[linha][2] == jogador {
        return StatusJogo::Vitoria(jogador);
    }
    // Verifica a coluna da jogada
    if tabuleiro.matriz[0][coluna] == jogador &&
       tabuleiro.matriz[1][coluna] == jogador &&
       tabuleiro.matriz[2][coluna] == jogador {
        return StatusJogo::Vitoria(jogador);
    }
    // Verifica as diagonais só se a jogada foi em uma delas
    // Diagonal principal (0,0), (1,1), (2,2). A condição é linha == coluna.
    if linha == coluna {
        if tabuleiro.matriz[0][0] == jogador &&
           tabuleiro.matriz[1][1] == jogador &&
           tabuleiro.matriz[2][2] == jogador {
            return StatusJogo::Vitoria(jogador);
        }
    }

    // Diagonal secundária (0,2), (1,1), (2,0). A condição é linha + coluna == 2.
    if linha + coluna == 2 {
        if tabuleiro.matriz[0][2] == jogador &&
           tabuleiro.matriz[1][1] == jogador &&
           tabuleiro.matriz[2][0] == jogador {
            return StatusJogo::Vitoria(jogador);
        }
    }

    // Tabuleiro cheio - Se ninguém venceu, verifica se há empate
    if !tabuleiro.matriz.iter().any(|linha| linha.contains(&'.')) {
        return StatusJogo::Empate;
    }

    // Tabuleiro não cheio - Se ninguém venceu o jogo continua
    StatusJogo::EmAndamento
}