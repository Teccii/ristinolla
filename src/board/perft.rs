#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[inline]
    fn perft(board: &Board, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let move_list = board.gen_moves();

        if depth == 1 {
            nodes += move_list.len() as u64;
        } else {
            for &mv in move_list.iter() {
                let mut board = board.clone();
                board.make_move(mv);

                nodes += perft(&board, depth - 1);
            }
        }

        nodes
    }

    macro_rules! perft_test {
        ($name:ident: $board:expr; $($nodes:expr),*) => {
            #[test]
            fn $name() {
                const NODES: &'static [u64] = &[$($nodes),*];

                let board = Board::from_fen($board).unwrap();
                for (depth, &nodes) in NODES.iter().enumerate() {
                    let perft_nodes = perft(&board, depth as u8);
                    assert_eq!(perft_nodes, nodes, "Depth: {} Expected: {} Got: {}", depth, nodes, perft_nodes);
                }
            }
        };
    }

    perft_test!(
        perft_startpos: "9/9/9/9/9/9/9/9/9 x 0 -";
        1, 81, 720, 6336, 55080, 473256, 4020960, 33782544, 281067408
    );

    perft_test!(
        perft_pos1: "9/3x5/3oo4/4x2o1/1o7/3x1o3/4o2x1/1x3x3/ox3o1x1 x 0 h6";
        1, 6, 44, 320, 2278, 16233, 116635, 849418, 6381392
    );

    perft_test!(
        perft_pos2: "4oo1x1/9/8x/9/3x5/o2x5/1xo6/1o5o1/xo1x1x1o1 x 0 e9";
        1, 7, 51, 370, 2827, 21383, 179029, 1487259, 13593237
    );

    perft_test!(
        perft_pos3: "1x7/6ox1/o2o4o/5o3/9/4o4/2xx2o1x/4x4/1ox3x2 X 0 g8";
        1, 9, 64, 454, 3185, 23060, 166468, 1260336, 9736622
    );

    perft_test!(
        perft_pos4: "1x3o3/7x1/5o2x/1x3o3/5o1x1/o2o5/3o5/xx7/1x4o2 X 0 f7";
        1, 8, 58, 463, 3479, 29053, 241143, 2173280, 19748086
    );

    perft_test!(
        perft_pos5: "7o1/3xx2xx/1x1o2x1x/2oo1o2o/1x2oxoo1/1x3xo2/1xx1oo3/2x2oo1x/o3x3o X 0 f2";
        1, 44, 391, 3436, 31662, 289755, 2792347, 26647358, 264767468
    );

    perft_test!(
        perft_pos6: "3o1o3/2xo1x1xx/xox6/6xxo/8o/2x2oxxo/ox2o1ox1/2o5o/o2xox1xo X 0 f4";
        1, 4, 28, 239, 2212, 21384, 196693, 1923003, 18155997
    );

    perft_test!(
        perft_pos7: "o1xxox2o/2ox2o2/x3o1o1x/3xx2o1/2x4x1/o2o2x2/oxx1o4/1o2o3o/xxo1x4 X 0 c1";
        1, 8, 86, 694, 5205, 40777, 319881, 2664061, 22872400
    );

    perft_test!(
        perft_pos8: "2ox4x/o2xx1x1o/o2x2oo1/1ox2o3/2oo3x1/x1xo3x1/x1xxo2o1/1xo3o2/2o5x X 0 c5";
        1, 7, 67, 840, 9609, 115330, 1283277, 14818322, 158683651
    );

    perft_test!(
        perft_pos9: "ooox5/x5ox1/o2xo2o1/1xxx1xoo1/2x2o1ox/o3x3o/x2x5/ooxo1x3/6x2 X 0 g6";
        1, 41, 440, 4759, 48816, 496752, 4825482, 47240207, 442983131
    );

    perft_test!(
        perft_pos10: "xox3x2/o2ox1oo1/3x4x/4x2oo/xxx2oo1x/xox1o4/o1o2xoo1/5x3/1o1x5 X 0 b9";
        1, 6, 33, 298, 2978, 27462, 251373, 2277374, 20505230
    );

    perft_test!(
        perft_pos11: "1o1x1o2x/o1x2oxox/2xxx1ox1/o3xoxo1/xxx2oooo/o1o1xo1xx/2o2o1x1/x2o2xxx/xxoooo2o X 0 f6";
        1, 3, 22, 170, 1292, 7611, 42488, 178604, 683640
    );

    perft_test!(
        perft_pos12: "o3xxo1o/xxxxoxxoo/1x1xx1xx1/1o2oxooo/1o2oxxxo/1oxx2xx1/xo1ooo1ox/o1x2o3/oo7 X 0 h8";
        1, 4, 58, 519, 4456, 33205, 232391, 1384237, 7568559
    );

    perft_test!(
        perft_pos13: "1xxxoxxoo/1xx2xo1x/oxox1oo1x/oo1o2o1o/2xx1o1ox/1ooo3oo/o1xx1o3/x2xxoxxx/7ox X 0 g7";
        1, 6, 63, 414, 2614, 17476, 108288, 680618, 3769073
    );

    perft_test!(
        perft_pos14: "x1o1ox1xo/xo3ooxx/3o1xoxx/1oxoxx2o/xxxoxooo1/xo3o2x/1x1x1o2o/1oxox1o2/1x4oox X 0 f8";
        1, 5, 23, 171, 1094, 7508, 47807, 322940, 2032799
    );

    perft_test!(
        perft_pos15: "1xox1xoo1/x1o2oo2/ooxo1oo2/x2o2xox/1ooxox1ox/1x3x2x/1x1x1xoxx/oxx2x2o/xx1o1ooo1 X 0 d7";
        1, 22, 163, 1457, 10431, 82349, 519427, 3451682, 17775153
    );
}
