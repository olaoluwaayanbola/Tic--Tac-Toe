use std::io;
fn main() {
    let mut board = [[" ", " ", " "], [" ", " ", " "], [" ", " ", " "]];

    // Main game loop
    for i in 0..9 {
        println!(" {} | {} | {} ", board[0][0], board[0][1], board[0][2]);
        println!("-----------");
        println!(" {} | {} | {} ", board[1][0], board[1][1], board[1][2]);
        println!("-----------");
        println!(" {} | {} | {} ", board[2][0], board[2][1], board[2][2]);

        let player = if i % 2 == 0 { "X" } else { "O" };

        println!("{}'s turn. Enter row and column ", player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut words = input.split_whitespace();

        let row: usize = words.next().unwrap().parse().unwrap();
        let col: usize = words.next().unwrap().parse().unwrap();

        board[row - 1][col - 1] = player;

        // conditions to check for a win
        if (board[0][0] == player && board[0][1] == player && board[0][2] == player)
            || (board[1][0] == player && board[1][1] == player && board[1][2] == player)
            || (board[2][0] == player && board[2][1] == player && board[2][2] == player)
            || (board[0][0] == player && board[1][0] == player && board[2][0] == player)
            || (board[0][1] == player && board[1][1] == player && board[2][1] == player)
            || (board[0][2] == player && board[1][2] == player && board[2][2] == player)
            || (board[0][0] == player && board[1][1] == player && board[2][2] == player)
            || (board[0][2] == player && board[1][1] == player && board[2][0] == player)
        {
            println!("{} wins!", player);
            break;
        }
    }
}