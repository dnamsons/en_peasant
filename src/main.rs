use std::convert::TryInto;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Piece {
    Rook(Color),
    Queen(Color),
    King(Color),
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
}

impl Piece {
    fn from_char(c: char) -> Option<Piece> {
        let color = if c.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        };

        match c.to_ascii_lowercase() {
            'p' => Some(Piece::Pawn(color)),
            'r' => Some(Piece::Rook(color)),
            'n' => Some(Piece::Knight(color)),
            'b' => Some(Piece::Bishop(color)),
            'q' => Some(Piece::Queen(color)),
            'k' => Some(Piece::King(color)),
            _ => None,
        }
    }
}

type Row = [Option<Piece>; 8];
type BoardContent = [Row; 8];

#[derive(Debug)]
struct Board {
    content: BoardContent,
    metadata: String,
}

const INITIAL_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Board {
    fn new() -> Self {
        Board::from_fen(String::from(INITIAL_FEN)).unwrap()
    }

    fn from_fen(pgn: String) -> Result<Self, &'static str> {
        let a = pgn.split_once(' ').unwrap();
        let (rows_string, metadata) = a;
        let rows = rows_string.split('/');

        let row_array: Vec<Row> = rows
            .map(|row| {
                let mut row_array: Vec<Option<Piece>> = Vec::new();

                row.chars().for_each(|c| {
                    if c.is_ascii_digit() {
                        let digit = c.to_digit(10).unwrap();
                        assert!(digit > 0 && digit <= 8);

                        let mut empty_places = vec![None; digit.try_into().unwrap()];
                        row_array.append(&mut empty_places);
                    } else if c.is_ascii_alphabetic() {
                        row_array.push(Piece::from_char(c))
                    }
                });

                row_array.try_into().unwrap()
            })
            .collect();

        Ok(Board {
            content: row_array.try_into().unwrap(),
            metadata: metadata.to_string(),
        })
    }
}

fn main() {
    let board = Board::new();

    println!("{:?}", board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_from_char() {
        assert_eq!(Piece::from_char('P'), Some(Piece::Pawn(Color::White)));
        assert_eq!(Piece::from_char('p'), Some(Piece::Pawn(Color::Black)));
        assert_eq!(Piece::from_char('M'), None);
    }

    #[test]
    fn test_board_new() {
        let actual = Board::new().content;
        let expected: BoardContent = vec![
            vec![
                Some(Piece::Rook(Color::Black)),
                Some(Piece::Knight(Color::Black)),
                Some(Piece::Bishop(Color::Black)),
                Some(Piece::Queen(Color::Black)),
                Some(Piece::King(Color::Black)),
                Some(Piece::Bishop(Color::Black)),
                Some(Piece::Knight(Color::Black)),
                Some(Piece::Rook(Color::Black)),
            ]
            .try_into()
            .unwrap(),
            vec![
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
            ]
            .try_into()
            .unwrap(),
            vec![None; 8].try_into().unwrap(),
            vec![None; 8].try_into().unwrap(),
            vec![None; 8].try_into().unwrap(),
            vec![None; 8].try_into().unwrap(),
            vec![
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
            ]
            .try_into()
            .unwrap(),
            vec![
                Some(Piece::Rook(Color::White)),
                Some(Piece::Knight(Color::White)),
                Some(Piece::Bishop(Color::White)),
                Some(Piece::Queen(Color::White)),
                Some(Piece::King(Color::White)),
                Some(Piece::Bishop(Color::White)),
                Some(Piece::Knight(Color::White)),
                Some(Piece::Rook(Color::White)),
            ]
            .try_into()
            .unwrap(),
        ]
        .try_into()
        .unwrap();

        assert_eq!(actual, expected);
    }
}
