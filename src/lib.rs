// State is a vector of living cells
//
// Update state by checking all living cells and create a new vec of coords + counts at those
// coords of all 8 cells around each living one
//
// Go through living and check if in new vec
// If not, kill
// Else, check rules (to kill/not kill in living) and then kill from new vec
//
// Next check all elts of new vec,
// Apply dead rules and add new elt to state if necessary
//
//
//
// May want to know if checking and updating living cells first then updating new
// is actually better than reforming living every time and only checking new

use std::collections::HashMap;

pub struct Game {
    living: Vec<(i32, i32)>,
}

impl Game {
    pub fn new(living: Vec<(i32, i32)>) -> Game {
        Game { living }
    }

    pub fn update(&mut self) {
        let mut map = HashMap::new();
        let mut ret = Vec::new();

        for cell in &self.living {
            // Add to HashMap with val of 1 or update with +1
            for i in -1..=1 {
                for j in -1..=1 {
                    if (i, j) != (0, 0) {
                        let count = map.entry((cell.0 + i, cell.1 + j)).or_insert(0);
                        *count += 1;
                    }
                }
            }
        }

        for (&key, &value) in &map {
            if value == 3 {
                ret.push(key);
            }
            if value == 2 && self.is_alive(&key) {
                ret.push(key);
            }
        }

        self.living = ret;
    }

    // TEMP DISPLAY FUNCTION
    pub fn display(&self) {
        for i in -5..=5 {
            for j in -5..=5 {
                if !self.is_alive(&(i, j)) {
                    print!("O");
                } else {
                    print!("X");
                }
            }
            println!();
        }
        println!();
    }
}

impl Game {
    fn is_alive(&self, cell: &(i32, i32)) -> bool {
        self.living.contains(cell)
    }
}
