use crossterm::{
    cursor::MoveToColumn,
    event::{
        DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode, KeyModifiers, read,
    },
    execute,
    style::{Color, ResetColor, SetForegroundColor, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::Rng;
use std::io::{self, stdout};

fn move_blocks_row(board: &mut [i16; 4]) {
    for col in 1..board.len() {
        if board[col] != 0 && board[col - 1] == 0 {
            board[col - 1] = board[col];
            board[col] = 0;
            move_blocks_row(board);
        }
    }
}

fn reverse_board(board: &mut [[i16; 4]; 4]) {
    for row in board.iter_mut() {
        row.reverse();
    }
}

fn transpose_board(board: &mut [[i16; 4]; 4]) {
    for i in 0..board.len() {
        for j in i + 1..board.len() {
            let tmp = board[i][j];
            board[i][j] = board[j][i];
            board[j][i] = tmp;
        }
    }
}

fn collapse_blocks(board: &mut [[i16; 4]; 4], direction: &str) {
    if direction == "Left" {
        let shadow_board = board.clone();
        for row in 0..board.len() {
            move_blocks_row(&mut board[row]);
            for col in 1..board[row].len() {
                if board[row][col - 1] == board[row][col] {
                    board[row][col - 1] = board[row][col - 1] * 2;
                    board[row][col] = 0;
                }
            }
            move_blocks_row(&mut board[row]);
        }
        //Check if the board changed at all. If so, add a new block
        if shadow_board != *board {
            add_new_block(board);
        }
    }
    if direction == "Right" {
        reverse_board(board);
        collapse_blocks(board, "Left");
        reverse_board(board);
    }
    if direction == "Up" {
        transpose_board(board);
        collapse_blocks(board, "Left");
        transpose_board(board);
    }
    if direction == "Down" {
        transpose_board(board);
        reverse_board(board);
        collapse_blocks(board, "Left");
        reverse_board(board);
        transpose_board(board);
    }
}

fn add_new_block(board: &mut [[i16; 4]; 4]) {
    // get locations of all zeroes on the board and make a list of them
    let mut blank_cells = Vec::new();
    let mut rng = rand::rng();
    for (index, _value) in board.iter().enumerate() {
        let row = index;
        for (index, value) in board[row].iter().enumerate() {
            let col = index;
            if *value == 0 {
                blank_cells.push((row, col));
            }
        }
    }
    // check for game over
    if blank_cells.is_empty() {
        //game over
        return;
    }
    // randomly choose one location
    if blank_cells.len() > 1 {
        let idx = rng.random_range(0..blank_cells.len() - 1);
        let (row, col) = blank_cells[idx];
        let val = if rng.random_range(0..10) < 9 { 2 } else { 4 };
        // set value
        board[row][col] = val;
    } else {
        let idx = 0;
        let (row, col) = blank_cells[idx];
        let val = if rng.random_range(0..10) < 9 { 2 } else { 4 };
        // set value
        board[row][col] = val;
    }
}

fn color_tiles(value: i16) -> Color {
    match value {
        0 => Color::DarkGrey,
        2 => Color::White,
        4 => Color::Cyan,
        8 => Color::Blue,
        16 => Color::Green,
        32 => Color::Yellow,
        64 => Color::Magenta,
        128 => Color::Red,
        256 => Color::DarkRed,
        512 => Color::DarkYellow,
        1024 => Color::DarkGreen,
        2048 => Color::DarkMagenta,
        _ => Color::White,
    }
}

fn refresh_board(board: [[i16; 4]; 4]) {
    print!("{}[2J", 27 as char);
    println!("┏━━━━━━┯━━━━ 2048 ━━━┯━━━━━━┓\r");
    for (ridx, &row) in board.iter().enumerate() {
        print!("┃");
        for (cidx, &col) in row.iter().enumerate() {
            let color = color_tiles(col);
            if col == 0 {
                print!("{:^6}", "      ".with(color));
            } else {
                execute!(stdout(), SetForegroundColor(color)).unwrap();
                print!("{:^6}", col);
                execute!(stdout(), ResetColor).unwrap();
            }
            if (cidx + 1) < row.len().try_into().unwrap() {
                print!("│")
            }
        }
        print!("┃");
        print!("\n\r");
        if (ridx + 1) < board.len().try_into().unwrap() {
            println!("┠──────┼──────┼──────┼──────┨\r");
        }
    }
    println!("┗━━━━━━┷━━━━━━┷━━━━━━┷━━━━━━┛\r");
    println!("\nPress 'q' to quit.\r")
}

fn main() -> io::Result<()> {
    println!();
    execute!(
        stdout(),
        EnableBracketedPaste,
        EnableFocusChange,
        MoveToColumn(0),
        EnableMouseCapture
    )?;
    enable_raw_mode()?;
    const C: usize = 4;
    const R: usize = 4;
    let mut board: [[i16; C]; R] = [[0; C]; R];

    add_new_block(&mut board);
    add_new_block(&mut board);
    loop {
        refresh_board(board);

        // Listen for key events
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::Up => collapse_blocks(&mut board, "Up"),
                KeyCode::Char('w') => collapse_blocks(&mut board, "Up"),
                KeyCode::Down => collapse_blocks(&mut board, "Down"),
                KeyCode::Char('s') => collapse_blocks(&mut board, "Down"),
                KeyCode::Left => collapse_blocks(&mut board, "Left"),
                KeyCode::Char('a') => collapse_blocks(&mut board, "Left"),
                KeyCode::Right => collapse_blocks(&mut board, "Right"),
                KeyCode::Char('d') => collapse_blocks(&mut board, "Right"),
                _ => {}
            }
        }
    }
    execute!(
        stdout(),
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture,
    )?;
    disable_raw_mode()?;
    Ok(())
}
