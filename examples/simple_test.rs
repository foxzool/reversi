use reversi::game::{Board, PlayerColor};

fn main() {
    println!("Testing Reversi board...");
    
    let mut board = Board::new();
    println!("Initial board created");
    
    println!("Black pieces: {}", board.count_pieces(PlayerColor::Black));
    println!("White pieces: {}", board.count_pieces(PlayerColor::White));
    
    let valid_moves = board.get_valid_moves_list(PlayerColor::Black);
    println!("Valid moves for Black: {}", valid_moves.len());
    
    for (i, chess_move) in valid_moves.iter().enumerate() {
        let (row, col) = Board::position_to_coords(chess_move.position);
        println!("  Move {}: row {}, col {}", i+1, row, col);
    }
    
    if let Some(first_move) = valid_moves.first() {
        println!("Making first move...");
        board.make_move(first_move.position, PlayerColor::Black);
        
        println!("After move:");
        println!("Black pieces: {}", board.count_pieces(PlayerColor::Black));
        println!("White pieces: {}", board.count_pieces(PlayerColor::White));
    }
    
    println!("Test completed successfully!");
}