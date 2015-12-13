use std;
use nonoparse;

extern crate ncurses;



pub struct Renderer<'a> {
    board: &'a nonoparse::Board,
    cache: BoardDrawCache,
}


impl<'a> Drop for Renderer<'a> {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}


impl<'a> Renderer<'a> { // TODO: remove once https://github.com/rust-lang/rust/issues/15872 is fixed
    pub fn new(board: &nonoparse::Board) -> Renderer {
        ncurses::initscr();
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        Renderer { board: board, cache: create_cache(board), }
    }

    pub fn draw(&self) {
        ncurses::printw(&self.cache.top_ribbon);

        for i in 0..self.board.rows.len() {
            ncurses::printw(&self.cache.left_rows[i]);

            for cell in &self.board.goal[i] {
                if *cell {
                    if self.cache.scale == 0 {
                        ncurses::addch('X' as ncurses::chtype);
                    } else {
                        ncurses::addch('[' as ncurses::chtype);
                        for _ in 1..self.cache.scale {
                            ncurses::addch('=' as ncurses::chtype);
                        }
                        ncurses::addch(']' as ncurses::chtype);
                    }
                } else {
                    for _ in 0..(self.cache.scale + 1) {
                        ncurses::addch(' ' as ncurses::chtype);
                    }
                }
            }
            ncurses::addch('\n' as ncurses::chtype);
        }
    }
}


struct BoardDrawCache {
    top_ribbon: String,
    left_rows: Vec<String>,
    scale: usize,
}


fn create_cache(board: &nonoparse::Board) -> BoardDrawCache {
    let mut cache = BoardDrawCache { top_ribbon: String::new(), left_rows: Vec::new(), scale: 0 };

    cache.scale = 1 + (((board.rows.len() - 1) as f64).log10() as usize);

    let max_y = board.cols.iter().fold(0, |max, col| std::cmp::max(max, col.len()));
    let row_widths: Vec<_> = board.rows.iter().map(|row| row.len() - 1 + row.iter().fold(0, |sum, v| sum + 1 + (*v as f64).log10() as usize)).collect();
    let max_row_width = *row_widths.iter().max().unwrap();

    let mut top_ribbon = Vec::new();
    for i in (0..max_y).rev() {
        let mut line = format!("{empty:0$}", max_row_width, empty="");

        for column in &board.cols {
            let format = if column.len() < max_y - i {
                format!("{empty:0$}", cache.scale + 1, empty="")
            } else {
                format!("{value:>0$}", cache.scale + 1, value=column[column.len() - (max_y - i)])
            };
            line = line + &format;
        }
        line.push_str("\n");

        top_ribbon.push(line);
    }
    cache.top_ribbon = top_ribbon.iter().rev().fold(String::new(), |sum, s| sum + s);

    for i in 0..board.rows.len() {
        let mut s = String::new();

        for k in board.rows[i].iter().rev() {
            let n = format!(" {}", k);
            s = n + &s;
        }

        if row_widths[i] != max_row_width {
            let padding = format!("{empty:0$}", (max_row_width - row_widths[i]), empty="");
            s = padding + &s;
        }

        cache.left_rows.push(s[1..].to_string());
    }

    return cache;
}
