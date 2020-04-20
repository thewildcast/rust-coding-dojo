// No need to specify an extern crate
// You can use it directly
use simple_matrix::Matrix;
use std::fmt;
use std::{thread, time};
use std::io::{self, Read};
use std::collections::LinkedList;

extern crate ncurses;
use ncurses::*; // watch for globs

enum Cell {
    Empty,
    Head, // *
    Body, // +
    Border,  // #
    Fruit,  // @
}

struct XY {
    x: i32,
    y: i32,
}

struct Game {
    food: XY,
    snake: LinkedList<XY>,
    // velocity: i32,
    direction: i32,
    width: usize,
    height: usize,

}

const ACTION_UP: i32 = 119;
const ACTION_DOWN: i32 = 115;
const ACTION_LEFT: i32 = 97;
const ACTION_RIGHT: i32 = 100;

impl Default for Cell{
    fn default() -> Self { Cell::Empty }
}

const ROWS: usize = 10;
const COLS: usize = 20;

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Empty => write!(f, " "),
            Cell::Head => write!(f, "*"),
            Cell::Body => write!(f, "+"),
            Cell::Border => write!(f, "#"),
            Cell::Fruit => write!(f, "@"),
        }
    }
    
}

impl Game {
    fn move_snake(&mut self, action: i32) {
        // TODO:
        // 1) move snake :P
        // 2) constraints: if moving up, you can't move down
        match action {
            ACTION_UP => write!(f, " "),
            ACTION_DOWN => write!(f, "*"),
            ACTION_LEFT => write!(f, "+"),
            ACTION_RIGHT => write!(f, "#"),
        }
    }
}

fn main() {
    let window = initscr();
    //printw("Snake");
    clear();
    noecho();
    // nodelay(window, true);
    let mut game = Game{
        food: XY{x: 1, y: 5},
        direction: ACTION_UP,
        snake: LinkedList::new(),
        width: COLS,
        height: ROWS,
    };
    
    let mut y  = (ROWS as i32)/2;
    let x  = (COLS as i32)/2;
    game.snake.push_front(XY{x, y});
    y -= 1;
    game.snake.push_front(XY{x, y});
    y -= 1;
    game.snake.push_front(XY{x, y});
    
    let mut ch: i32;

    // let sleep = time::Duration::from_millis(5000);
    // loop {
        // let ch = getch();
        print_board(&to_board(&game));
        // thread::sleep(sleep);
    // }
    // for {
         let ch = getch();
    // }

    /* Terminate ncurses. */
    endwin();

    /*
    let mut mat = initialize_board(ROWS, COLS);
    print_board(&mat);

    for pos in 1..9 {
        let sleep = time::Duration::from_millis(500);
        thread::sleep(sleep);
        mat.set(5,pos,Cell::Head);
        print_board(&mat);
    }
    */
}
// Trait matrix::Matrix
// impl<T: Element> Matrix for Conventional<T>
// Returns a board with food and snake printed
fn to_board(g: &Game) -> Matrix<Cell>{
    let mut m = initialize_board(g.height, g.width);

    // Builds snake body
    for e in g.snake.iter() {
        m.set(e.y as usize, e.x as usize, Cell::Body);
    }

    // Builds food cells
    m.set(g.food.y as usize, g.food.x as usize, Cell::Fruit);

    return m;
}

// Returns an empty board with given cols and rows
fn initialize_board(rows: usize, cols: usize) -> Matrix<Cell> {
    let mut m = Matrix::new(rows, cols);
    for r in 0..rows {
        m.set(r, 0, Cell::Border);
        m.set(r, cols-1, Cell::Border);
    }
    for c in 0..cols {
        m.set(0, c, Cell::Border);
        m.set(rows - 1, c, Cell::Border);
    }
    return m;
}



// Tablero de 80x24
// * = cabeza de la vibora
// + = el cuerpo de la vibora
// @ = fruta
fn print_board(mat: &Matrix<Cell>) {
    // printw("Snake");
    // refresh();
    clear();
    let mut i: i32 = 0;
    for y in 0..mat.rows() {
        for x in 0..mat.cols() {
            match mat.get(y, x) {
                None => continue,
                Some(cell) => {
                    mvprintw(y as i32, x as i32, format!("{}", cell).as_str());
                }
            };
        }
    }
}