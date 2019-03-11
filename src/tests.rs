use crate::board::{after_move, gen_moves, zobrist_hash, INITIAL_BOARD_STATE};
use crate::ui::{from_fen, parse_move, render_board, render_coordinates};

pub fn tests() -> bool {
    // Test FEN loading is coerent with move making
    let mut board_state = INITIAL_BOARD_STATE;

    let sicilian_moves = vec![
        "e2e4", "f2f4", "g1f3", "g1f3", "d2d4", "f4e5", "f3d4", "f3e5", "d1d4", "d2d3", "b1c3",
        "h2h3", "c1e3",
    ];
    let sicilian_fens = vec![
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
        "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3",
        "r1bqkbnr/pp1ppppp/2n5/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq d3 0 3",
        "r1bqkbnr/pp1ppppp/2n5/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 4",
        "r1bqkbnr/pp1ppppp/2n5/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 4",
        "r1bqkbnr/pp1ppppp/8/8/3nP3/8/PPP2PPP/RNBQKB1R w KQkq - 0 5",
        "r1bqkbnr/pp1ppppp/8/8/3QP3/8/PPP2PPP/RNB1KB1R b KQkq - 0 5",
        "r1bqkbnr/pp1p1ppp/4p3/8/3QP3/8/PPP2PPP/RNB1KB1R w KQkq - 0 6",
        "r1bqkbnr/pp1p1ppp/4p3/8/3QP3/2N5/PPP2PPP/R1B1KB1R b KQkq - 1 6",
        "r1bqkbnr/1p1p1ppp/p3p3/8/3QP3/2N5/PPP2PPP/R1B1KB1R w KQkq - 0 7",
    ];
    let sicilian_possible_moves = vec![
        vec![
            "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3",
            "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "g1h3", "g1f3",
        ],
        vec![
            "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3",
            "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "g1h3", "g1f3",
        ],
        vec![
            "e4e5", "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "f2f3", "f2f4",
            "g2g3", "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "d1e2", "d1f3", "d1g4", "d1h5", "e1e2",
            "f1e2", "f1d3", "f1c4", "f1b5", "f1a6", "g1h3", "g1f3", "g1e2",
        ],
        vec![
            "f4f5", "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4",
            "g2g3", "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "e1f2", "e1g3", "e1h4", "g1h3", "g1f3",
        ],
        vec![
            "e4e5", "f3g5", "f3e5", "f3d4", "f3g1", "f3h4", "a2a3", "a2a4", "b2b3", "b2b4", "c2c3",
            "c2c4", "d2d3", "d2d4", "g2g3", "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "d1e2", "e1e2",
            "f1e2", "f1d3", "f1c4", "f1b5", "f1a6", "h1g1",
        ],
        vec![
            "f4f5", "f4e5", "f3g5", "f3e5", "f3d4", "f3g1", "f3h4", "a2a3", "a2a4", "b2b3", "b2b4",
            "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "g2g3", "g2g4", "h2h3", "h2h4", "b1c3",
            "b1a3", "e1f2", "e1g3", "e1h4", "h1g1",
        ],
        vec![
            "e4e5", "f3g5", "f3e5", "f3d4", "f3d2", "f3g1", "f3h4", "a2a3", "a2a4", "b2b3", "b2b4",
            "c2c3", "c2c4", "g2g3", "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "b1d2", "c1d2", "c1e3",
            "c1f4", "c1g5", "c1h6", "d1d2", "d1d3", "d1d4", "d1e2", "e1e2", "e1d2", "f1e2", "f1d3",
            "f1c4", "f1b5", "f1a6", "h1g1",
        ],
        vec![
            "f3g5", "f3e5", "f3d4", "f3g1", "f3h4", "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4",
            "d2d3", "d2d4", "e2e3", "e2e4", "g2g3", "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "e1f2",
            "e1g3", "e1h4", "h1g1",
        ],
        vec![
            "e4e5", "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "f2f3", "f2f4", "g2g3", "g2g4",
            "h2h3", "h2h4", "b1c3", "b1a3", "b1d2", "c1d2", "c1e3", "c1f4", "c1g5", "c1h6", "d1d2",
            "d1d3", "d1d4", "d1e2", "d1f3", "d1g4", "d1h5", "e1e2", "e1d2", "f1e2", "f1d3", "f1c4",
            "f1b5", "f1a6", "h1g1",
        ],
        vec![
            "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "g2g3",
            "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "e1f2", "e1g3", "e1h4", "h1g1",
        ],
        vec![
            "d4d5", "d4d6", "d4d7", "d4c4", "d4b4", "d4a4", "d4d3", "d4d2", "d4d1", "d4e5", "d4f6",
            "d4g7", "d4c5", "d4b6", "d4a7", "d4c3", "d4e3", "e4e5", "a2a3", "a2a4", "b2b3", "b2b4",
            "c2c3", "c2c4", "f2f3", "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1c3", "b1a3", "b1d2",
            "c1d2", "c1e3", "c1f4", "c1g5", "c1h6", "e1e2", "e1d1", "e1d2", "f1e2", "f1d3", "f1c4",
            "f1b5", "f1a6", "h1g1",
        ],
        vec![
            "d3d4", "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "e2e3", "e2e4", "g2g3", "g2g4",
            "h2h3", "h2h4", "b1c3", "b1a3", "b1d2", "c1d2", "c1e3", "c1f4", "c1g5", "c1h6", "d1d2",
            "e1f2", "e1g3", "e1h4", "e1d2", "e1c3", "e1b4", "e1a5", "h1g1",
        ],
        vec![
            "d4d5", "d4d6", "d4d7", "d4c4", "d4b4", "d4a4", "d4d3", "d4d2", "d4d1", "d4e5", "d4f6",
            "d4g7", "d4c5", "d4b6", "d4a7", "d4e3", "e4e5", "c3d5", "c3b5", "c3a4", "c3b1", "c3d1",
            "c3e2", "a2a3", "a2a4", "b2b3", "b2b4", "f2f3", "f2f4", "g2g3", "g2g4", "h2h3", "h2h4",
            "a1b1", "c1d2", "c1e3", "c1f4", "c1g5", "c1h6", "e1e2", "e1d1", "e1d2", "f1e2", "f1d3",
            "f1c4", "f1b5", "f1a6", "h1g1",
        ],
    ];

    for ((fen, next_move), move_list) in sicilian_fens
        .iter()
        .zip(sicilian_moves)
        .zip(sicilian_possible_moves)
    {
        assert_eq!(render_board(&board_state), render_board(&from_fen(fen)));
        assert_eq!(zobrist_hash(&board_state), zobrist_hash(&from_fen(fen)));
        assert_eq!(
            move_list,
            gen_moves(&board_state)
                .iter()
                .map(|(from, to)| format!(
                    "{}{}",
                    render_coordinates(*from),
                    render_coordinates(*to)
                ))
                .collect::<Vec<_>>()
        );
        board_state = after_move(&board_state, &parse_move(next_move));
    }

    let fens = vec![
        "r1b1k2r/3n1p1p/p2PpnpR/qpp1p3/5P2/2N5/PPPQB1P1/1K1R2N1 w kq - 0 16",
        "7k/7p/8/1p5R/1P6/2Pb4/1r4PK/8 w - - 1 42",
        "8/5p1p/2p1p1pk/4Q3/7P/5qP1/r4P2/2R3K1 b - - 0 34",
        "rnbqkb1r/1p2pppp/p4n2/3pP3/2B5/2N2N2/PP3PPP/R1BQK2R w KQkq - 0 8",
        "rnbqk2r/1p1n1ppp/p3p3/2b1P3/8/1BN2N2/PP3PPP/R1BQK2R w KQkq - 2 11",
        "rnbqk2r/1p1n1ppp/p3p3/2b1P3/8/1BN2N2/PP3PPP/R1BQ1RK1 b kq - 3 11",
        "4rrk1/pp2b1pp/3p4/2pP3n/3N4/2NP1P2/PP1K1P1P/R6R w - c6 0 19",
    ];

    let possible_moves = vec![
        vec![
            "h6h7", "h6g6", "h6h5", "h6h4", "h6h3", "h6h2", "h6h1", "f4f5", "f4e5", "c3d5", "c3b5",
            "c3a4", "c3e4", "a2a3", "a2a4", "b2b3", "b2b4", "d2d3", "d2d4", "d2d5", "d2e3", "d2c1",
            "d2e1", "e2f3", "e2g4", "e2h5", "e2d3", "e2c4", "e2b5", "e2f1", "g2g3", "g2g4", "b1a1",
            "b1c1", "d1c1", "d1e1", "d1f1", "g1h3", "g1f3",
        ],
        vec![
            "h5h6", "h5h7", "h5g5", "h5f5", "h5e5", "h5d5", "h5c5", "h5b5", "h5h4", "h5h3", "c3c4",
            "g2g3", "g2g4", "h2h3", "h2h1", "h2g3", "h2g1",
        ],
        vec![
            "h7h8", "h7g7", "h7f7", "h7e7", "h7d7", "h7c7", "h7h6", "h7h5", "h7h4", "h7h3", "h7h2",
            "h7h1", "c6c7", "c6b6", "c6c5", "c6c4", "c6c3", "c6d6", "c6e6", "c6f6", "c6g6", "c6h6",
            "c6d7", "c6e8", "c6b7", "c6a8", "c6b5", "c6a4", "c6d5", "c6e4", "a3a4", "a3b4", "a3b2",
            "b3b4", "f3f4", "c2c3", "c2c4",
        ],
        vec![
            "e5e6", "e5f6", "c4d5", "c4b5", "c4a6", "c4b3", "c4d3", "c4e2", "c4f1", "c3d5", "c3b5",
            "c3a4", "c3b1", "c3e2", "c3e4", "f3g5", "f3d4", "f3d2", "f3g1", "f3h4", "a2a3", "a2a4",
            "b2b3", "b2b4", "g2g3", "g2g4", "h2h3", "h2h4", "a1b1", "c1d2", "c1e3", "c1f4", "c1g5",
            "c1h6", "d1d2", "d1d3", "d1d4", "d1d5", "d1e2", "d1c2", "d1b3", "d1a4", "e1e2", "e1f1",
            "e1d2", "h1g1", "h1f1", "e1g1",
        ],
        vec![
            "b3c4", "b3d5", "b3e6", "b3a4", "b3c2", "c3d5", "c3b5", "c3a4", "c3b1", "c3e2", "c3e4",
            "f3g5", "f3d4", "f3d2", "f3g1", "f3h4", "a2a3", "a2a4", "g2g3", "g2g4", "h2h3", "h2h4",
            "a1b1", "c1d2", "c1e3", "c1f4", "c1g5", "c1h6", "d1d2", "d1d3", "d1d4", "d1d5", "d1d6",
            "d1d7", "d1e2", "d1c2", "e1e2", "e1f1", "e1d2", "h1g1", "h1f1", "e1g1",
        ],
        vec![
            "f4g5", "f4h6", "f4e5", "f4d6", "f4c7", "f4e3", "f4d2", "f4c1", "f4g3", "f4h2", "h3h4",
            "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "e2d4", "e2c3", "e2c1", "e2g3", "g2g3",
            "g2g4", "a1b1", "a1c1", "d1b1", "d1d2", "d1c1", "e1f2", "e1g3", "e1h4", "e1d2", "e1c3",
            "e1b4", "e1a5", "g1f3", "h1h2",
        ],
        vec![
            "d5c6", "d4e6", "d4c6", "d4b5", "d4b3", "d4c2", "d4e2", "d4f5", "c3b5", "c3a4", "c3b1",
            "c3d1", "c3e2", "c3e4", "f3f4", "a2a3", "a2a4", "b2b3", "b2b4", "d2c2", "d2d1", "d2e2",
            "d2e3", "d2c1", "d2e1", "h2h3", "h2h4", "a1b1", "a1c1", "a1d1", "a1e1", "a1f1", "a1g1",
            "h1g1", "h1f1", "h1e1", "h1d1", "h1c1", "h1b1",
        ],
    ];

    for (fen, movelist) in fens.iter().zip(possible_moves) {
        board_state = from_fen(fen);
        assert_eq!(
            movelist,
            gen_moves(&board_state)
                .iter()
                .map(|(from, to)| format!(
                    "{}{}",
                    render_coordinates(*from),
                    render_coordinates(*to)
                ))
                .collect::<Vec<_>>(),
        );
    }

    true
}