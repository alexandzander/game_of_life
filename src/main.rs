use game_of_life::Game;

fn main() {
    let mut game = Game::new(vec![(-1, 0), (-1, -1), (0, -2), (1, 1), (2, 0), (2, -1)]);
    loop {
        game.display();
        game.update();
        wait();
    }
}

fn wait() {
    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(1));
}
