use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;



pub struct Board {
    pub rows: Vec<Vec<usize>>,
    pub cols: Vec<Vec<usize>>,
    pub goal: Vec<Vec<bool>>,
}


pub fn parse(filename: &str) -> Board
{
    let mut reader = BufReader::new(File::open(filename).unwrap());

    let mut board = Board { rows: Vec::new(), cols: Vec::new(), goal: Vec::new() };

    let mut width = 0;
    let mut height = 0;

    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        {
            let tokens: Vec<&str>= line.split_whitespace().collect();

            if tokens.len() == 0 { continue; }

            match tokens[0] {
                "width"   => width  = usize::from_str_radix(tokens[1], 10).unwrap(),
                "height"  => height = usize::from_str_radix(tokens[1], 10).unwrap(),
                "rows"    => board.rows = parse_numbers(&mut reader, height),
                "columns" => board.cols = parse_numbers(&mut reader, width),
                "goal"    => board.goal = parse_goal(&tokens[1][1..tokens[1].len()], width, height),
                _ => (),
            };
        }

        line.clear();
    }

    board
}


fn parse_numbers<T: Read>(reader: &mut BufReader<T>, line_count: usize) -> Vec<Vec<usize>>
{
    let mut numbers: Vec<Vec<usize>> = Vec::with_capacity(line_count);

    let mut line = String::new();
    for i in 0..line_count {
        reader.read_line(&mut line).unwrap();
        numbers.push(Vec::new());

        for number in line.split(',') {
            numbers[i].push(usize::from_str_radix(number.trim(), 10).unwrap());
        }
        line.clear();
    }

    numbers
}


fn parse_goal(bits: &str, width: usize, height: usize) -> Vec<Vec<bool>>
{
    let mut goal: Vec<Vec<bool>> = Vec::with_capacity(width);

    let mut c = bits.chars();

    for x in 0..width {
        goal.push(Vec::with_capacity(height));

        for _ in 0..height {
            goal[x].push(c.next().unwrap() == '1');
        }
    }

    goal
}
