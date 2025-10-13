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
    // Inicializa um tabuleiro vazio (usando '.' como no seu exemplo)
    let mut tabuleiro = Tabuleiro {
        matriz: [
            ['.', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.'],
        ]
    };
    // A primeira jogada é sempre do 'X'
    let mut jogador_atual = 'X';

    println!("Bem-vindo ao Jogo da Velha em Rust!");

    // ● Laço para jogadas
    loop {
        // Pede a jogada do jogador atual e converte para coordenadas
        let (linha_u32, coluna_u32) = ler_posicao(&tabuleiro, jogador_atual);
        
        // Em Rust, a indexação de arrays é feita com 'usize', não 'u32'.
        // É importante fazer essa conversão (casting).
        let linha = linha_u32 as usize;
        let coluna = coluna_u32 as usize;

        // Validação extra: verifica se a célula escolhida já não está ocupada
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
            // ● Se um jogador ganhar, finalizar a função (com break)
            StatusJogo::Vitoria(vencedor) => {
                println!("\n--- FIM DE JOGO ---");
                mostrar_tabuleiro(&tabuleiro);
                println!("O jogador '{}' venceu!", vencedor);
                break; // Encerra o laço 'loop'
            },
            // ● Detectar empate e finalizar também
            StatusJogo::Empate => {
                println!("\n--- FIM DE JOGO ---");
                mostrar_tabuleiro(&tabuleiro);
                println!("O jogo terminou em empate!");
                break; // Encerra o laço 'loop'
            },
            // Se o jogo continua, passa para o próximo jogador
            StatusJogo::EmAndamento => {
                // ● Alterna o jogador em cada laço
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
        if i < 2 { // Imprime separador entre as linhas
            println!("---+---+---");
        }
    }
    println!();
}

fn converter_indice_para_coordenada(indice: u32) -> Option<(u32, u32)>{
    let indice_base_zero = indice - 1; //convertendo pra base 0 a 8
    let linha = indice_base_zero/ 3;
    let coluna = indice_base_zero % 3;
    Some((linha, coluna))
}

fn ler_posicao(tabuleiro: &Tabuleiro, jogador: char) -> (u32, u32) {
    loop {
        mostrar_tabuleiro(tabuleiro);
        println!("Jogador '{}', escolha a sua jogada (1-9):", jogador);

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erro de leitura. Tente novamente");
            continue;
        }

        let indice: u32 = match input.trim().parse() {
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
                // O loop continuará
            }
        }
    }
}

fn verificar_fim_da_partida(tabuleiro: &Tabuleiro, linha: usize, coluna: usize) -> StatusJogo {
    // 1. Pega o caractere do jogador que acabou de fazer a jogada
    let jogador = tabuleiro.matriz[linha][coluna];

    // Se o espaço estiver vazio ('.'), algo está errado, mas o jogo continua
    if jogador == '.' {
        return StatusJogo::EmAndamento;
    }

    // 2. Verifica a linha da jogada
    if tabuleiro.matriz[linha][0] == jogador &&
       tabuleiro.matriz[linha][1] == jogador &&
       tabuleiro.matriz[linha][2] == jogador {
        return StatusJogo::Vitoria(jogador);
    }

    // 3. Verifica a coluna da jogada
    if tabuleiro.matriz[0][coluna] == jogador &&
       tabuleiro.matriz[1][coluna] == jogador &&
       tabuleiro.matriz[2][coluna] == jogador {
        return StatusJogo::Vitoria(jogador);
    }

    // 4. Verifica as diagonais, mas SÓ SE a jogada foi em uma delas
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

    // 5. Se ninguém venceu, verifica se há empate (tabuleiro cheio)
    if !tabuleiro.matriz.iter().any(|linha| linha.contains(&'.')) {
        return StatusJogo::Empate;
    }

    // 6. Se ninguém venceu e o tabuleiro não está cheio, o jogo continua
    StatusJogo::EmAndamento
}