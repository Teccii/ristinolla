use crate::{
    board::{Board, MoveList},
    search::SearchLimit,
    types::Square,
};
use std::{
    iter::Peekable,
    num::ParseIntError,
    str::{FromStr, ParseBoolError, SplitAsciiWhitespace},
};

#[derive(Debug, Clone)]
pub enum UgiCommand {
    Ugi,
    NewGame,
    IsReady,
    Display,
    Position { board: Board, moves: Vec<Square> },
    Search(Vec<SearchLimit>),
    Perft { depth: u8, bulk: bool },
    SplitPerft { depth: u8, bulk: bool },
    Bench { depth: u8 },
    SetOption { name: String, value: String },
    Wait,
    Stop,
    Quit,
}

/*----------------------------------------------------------------*/

#[derive(Debug, Clone, thiserror::Error)]
pub enum UgiParseError {
    #[error("Missing Command")]
    MissingCommand,
    #[error("Unknown Command: `{0}`")]
    UnknownCommand(String),
    #[error("Invalid FEN: `{0}`")]
    InvalidFen(String),
    #[error("Invalid Move: `{0}`")]
    InvalidMove(String),
    #[error("Missing position type (e.g. `startpos`, `fen`) in `position` command")]
    MissingPositionType,
    #[error("Missing `moves` token in `position` command")]
    MissingPositionMovesToken,
    #[error("Unknown Search Limit: `{0}`")]
    UnknownLimit(String),
    #[error("Missing Search Limit Value: `{0}`")]
    MissingLimitValue(String),
    #[error("Missing depth option in `perft` or `splitperft` command")]
    MissingPerftDepth,
    #[error("Missing bulk option in `perft` or `splitperft` command")]
    MissingPerftBulk,
    #[error("Missing `name` token in `setoption` command")]
    MissingOptionNameToken,
    #[error("Missing `value` token in `setoption` command")]
    MissingOptionValueToken,
    #[error("Missing option name in `setoption` command")]
    MissingOptionName,
    #[error("Missing option value in `setoption` command")]
    MissingOptionValue,
    #[error("Error parsing integer: `{0}`")]
    InvalidInteger(#[from] ParseIntError),
    #[error("Error parsing bool: `{0}`")]
    InvalidBool(#[from] ParseBoolError),
}

/*----------------------------------------------------------------*/

impl UgiCommand {
    #[inline]
    pub fn parse(input: &str, board: &Board) -> Result<UgiCommand, UgiParseError> {
        use UgiCommand::*;
        use UgiParseError::*;

        let mut reader = input.split_ascii_whitespace();
        let cmd = reader.next().ok_or(MissingCommand)?;

        match cmd {
            "ugi" => Ok(Ugi),
            "uginewgame" => Ok(NewGame),
            "isready" => Ok(IsReady),
            "display" | "d" => Ok(Display),
            "wait" => Ok(Wait),
            "stop" => Ok(Stop),
            "quit" | "q" => Ok(Quit),
            "position" | "pos" => Self::parse_pos(reader),
            "go" => Self::parse_go(reader, board),
            "perft" => {
                let depth = reader.next().ok_or(MissingPerftDepth)?.parse::<u8>()?;
                let bulk = reader.next().ok_or(MissingPerftBulk)?.parse::<bool>()?;

                Ok(Perft { depth, bulk })
            }
            "splitperft" => {
                let depth = reader.next().ok_or(MissingPerftDepth)?.parse::<u8>()?;
                let bulk = reader.next().ok_or(MissingPerftBulk)?.parse::<bool>()?;

                Ok(SplitPerft { depth, bulk })
            }
            "bench" => {
                let depth = reader.next().map_or(Ok(5), str::parse)?;
                Ok(Bench { depth })
            }
            "setoption" => {
                if reader.next() != Some("name") {
                    return Err(MissingOptionNameToken);
                }

                let name = reader.next().ok_or(MissingOptionName)?.to_string();
                if reader.next() != Some("value") {
                    return Err(MissingOptionValueToken);
                }

                let value = reader.next().ok_or(MissingOptionValue)?.to_string();
                Ok(SetOption { name, value })
            }
            _ => Err(UnknownCommand(cmd.to_string())),
        }
    }

    #[inline]
    fn parse_pos(mut reader: SplitAsciiWhitespace) -> Result<UgiCommand, UgiParseError> {
        use UgiCommand::*;
        use UgiParseError::*;

        let startpos = match reader.next() {
            Some("startpos") => Board::default(),
            Some("fen") => {
                let mut fen = String::new();
                for part in reader.by_ref().take(4) {
                    if !fen.is_empty() {
                        fen.push(' ');
                    }

                    fen.push_str(part);
                }

                Board::from_fen(&fen).ok_or(InvalidFen(fen))?
            }
            _ => return Err(MissingPositionType),
        };

        if reader.next().is_some_and(|token| token != "moves") {
            return Err(MissingPositionMovesToken);
        }

        let mut current = startpos.clone();
        let mut moves = Vec::new();

        for token in reader {
            let mv = token
                .trim()
                .parse::<Square>()
                .map_err(|_| InvalidMove(token.to_string()))?;

            if !current.is_legal(mv) {
                return Err(InvalidMove(token.to_string()));
            }

            moves.push(mv);
            current.make_move(mv);
        }

        Ok(Position {
            board: startpos,
            moves,
        })
    }

    #[inline]
    fn parse_go(
        mut reader: SplitAsciiWhitespace,
        board: &Board,
    ) -> Result<UgiCommand, UgiParseError> {
        use SearchLimit::*;
        use UgiCommand::*;
        use UgiParseError::*;

        let keywords = &[
            "searchmoves",
            "xtime",
            "otime",
            "xinc",
            "oinc",
            "movetime",
            "movestogo",
            "depth",
            "nodes",
            "infinite",
        ];

        let mut reader = reader.peekable();
        let mut limits = Vec::new();

        #[inline]
        fn parse_int<T: FromStr<Err = ParseIntError>>(
            reader: &mut Peekable<SplitAsciiWhitespace>,
            token: &str,
        ) -> Result<T, UgiParseError> {
            Ok(reader
                .next()
                .ok_or_else(|| MissingLimitValue(token.to_string()))?
                .parse::<T>()?)
        }

        while let Some(token) = reader.next() {
            match token {
                "infinite" => {}
                "xtime" => limits.push(XTime(parse_int::<i64>(&mut reader, token)?.max(0) as u64)),
                "otime" => limits.push(OTime(parse_int::<i64>(&mut reader, token)?.max(0) as u64)),
                "xinc" => limits.push(XInc(parse_int(&mut reader, token)?)),
                "oinc" => limits.push(XInc(parse_int(&mut reader, token)?)),
                "movetime" => limits.push(MoveTime(parse_int(&mut reader, token)?)),
                "movestogo" => limits.push(MovesToGo(parse_int(&mut reader, token)?)),
                "depth" => limits.push(Depth(parse_int(&mut reader, token)?)),
                "nodes" => limits.push(Nodes(parse_int(&mut reader, token)?)),
                "searchmoves" => {
                    let mut moves = MoveList::new();
                    while let Some(token) = reader.peek()
                        && !keywords.contains(token)
                    {
                        let mv = token
                            .trim()
                            .parse::<Square>()
                            .map_err(|_| InvalidMove(token.to_string()))?;

                        if !board.is_legal(mv) {
                            return Err(InvalidMove(token.to_string()));
                        }

                        moves.push(mv);
                        reader.next();
                    }

                    limits.push(SearchMoves(moves))
                }
                _ => return Err(UnknownLimit(token.to_string())),
            }
        }

        Ok(Search(limits))
    }
}
