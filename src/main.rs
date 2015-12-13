pub mod nonoparse;
pub mod nonodraw;

extern crate ncurses;



fn main() {
    static KEY_ESC: i32 = 27;

    if std::env::args().len() < 2 {
        panic!("Please specify a .non file.");
    }

    let filename = std::env::args().nth(1).unwrap();

    let board = nonoparse::parse(&filename);

    let renderer = nonodraw::Renderer::new(&board);

    renderer.draw();

    while KEY_ESC != ncurses::getch() { }
}
