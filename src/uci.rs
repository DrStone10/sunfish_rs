use log::{info, trace, warn};
use std::time::Duration;

use crate::board::{after_move, gen_moves, A8, BOARD_SIZE, H8, INITIAL_BOARD_STATE};
use crate::pieces::{Piece, Square};
use crate::search::Searcher;
use crate::ui::{parse_move, render_move};

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    line
}

pub fn uci_loop() {
    let mut searcher = Searcher::default();
    println!("Sunfish_rs");
    let mut board_state = INITIAL_BOARD_STATE;
    let mut am_black = false;
    loop {
        let next_command = read_line();
        trace!("Received command {}", next_command);
        match next_command.split(' ').next().unwrap() {
            "quit" => return,
            "uci" => println!("uciok"),
            "isready" => println!("readyok"),
            "ucinewgame" => board_state = INITIAL_BOARD_STATE,
            "position" => {
                //position startpos moves d2d4 d7d5 e2e4 d5e4
                info!("loading moves");
                let moves: Vec<&str> = next_command.split(' ').collect();
                if moves.len() == 2 && moves[1] != "startpos" {
                    warn!("UNKNOWN FORMAT!");
                    panic!();
                } else if moves.len() > 2
                    && (moves[0] != "position" || moves[1] != "startpos" || moves[2] != "moves")
                {
                    warn!("UNKNOWN FORMAT!");
                    panic!();
                }
                board_state = INITIAL_BOARD_STATE;
                am_black = false;
                for move_ in moves.iter().skip(3) {
                    let mut parsed_move = parse_move(move_);
                    if am_black {
                        parsed_move.0 = BOARD_SIZE - 1 - parsed_move.0;
                        parsed_move.1 = BOARD_SIZE - 1 - parsed_move.1;
                    };
                    if !gen_moves(&board_state).contains(&parsed_move) {
                        warn!(
                            "Trying to make an illegal move {:?}, will probably fail",
                            parsed_move
                        );
                    }
                    board_state = after_move(&board_state, &parsed_move);
                    searcher.set_eval_to_zero(&board_state);
                    am_black = !am_black;
                }
                // print_board(&board_state);
            }
            "go" => {
                // TODO: refactor time management

                // Command format is going to be:
                // go wtime 391360 btime 321390 winc 8000 binc 8000
                let infos: Vec<&str> = next_command.split(' ').collect();
                let average_remaining_moves = 30;
                // Super basic time management
                let total_available_time: u64 = if infos.len() < 9 {
                    4_000 * average_remaining_moves
                } else if am_black {
                    infos[4].parse::<u64>().expect("Failed to parse time")
                        + infos[8].parse::<u64>().expect("Failed to parse time")
                            * average_remaining_moves
                } else {
                    infos[2].parse::<u64>().expect("Failed to parse time")
                        + infos[6].parse::<u64>().expect("Failed to parse time")
                            * average_remaining_moves
                };

                let mut nanos_for_move: u64 =
                    total_available_time * 1_000_000 / average_remaining_moves / 2;
                if nanos_for_move > 1_000_000_000 {
                    nanos_for_move -= 500_000_000 // Account for lag
                } else {
                    nanos_for_move = 200_000_000 // Minimum reasonable move time
                }

                let time_for_move = Duration::new(
                    nanos_for_move / 1_000_000_000,
                    (nanos_for_move % 1_000_000_000) as u32,
                );
                info!(
                    "Computing move giving time {:?} with {}s remaining",
                    time_for_move,
                    total_available_time / 1000
                );
                // TODO parse_movetime
                let (mut top_move, _score, _depth) =
                    searcher.search(board_state.clone(), time_for_move);
                let is_promotion = (A8 <= top_move.1 && top_move.1 <= H8)
                    && board_state.board[top_move.0] == Square::MyPiece(Piece::Pawn);
                if am_black {
                    top_move.0 = BOARD_SIZE - 1 - top_move.0;
                    top_move.1 = BOARD_SIZE - 1 - top_move.1;
                };
                if is_promotion {
                    println!("bestmove {}q ponder e7e5", render_move(&top_move));
                } else {
                    println!("bestmove {} ponder e7e5", render_move(&top_move));
                }
                info!("Sending bestmove {}", render_move(&top_move));
                info!(
                    "Searched {} nodes, reached depth {}, estimate score {}, tables at {} and {}",
                    searcher.nodes,
                    _depth,
                    _score,
                    searcher.move_transposition_table.len(),
                    searcher.score_transposition_table.len()
                );
            }
            _ => {
                warn!("UNKNOWN COMMAND {}", next_command);
                println!("Unknown command:{}", next_command);
            }
        }
    }
}
