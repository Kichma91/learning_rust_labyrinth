use std::ops::{IndexMut,Index};
use std::io;


struct Lab {
    r0: Vec<char>,
    r1: Vec<char>,
    r2: Vec<char>,
    r3: Vec<char>,
    r4: Vec<char>,
    r5: Vec<char>,
    r6: Vec<char>,
    r7: Vec<char>,
    r8: Vec<char>,
    r9: Vec<char>,
    pos_x: usize,
    pos_y: usize,
}

impl Index<usize> for Lab {
    type Output = Vec<char>;
    fn index(&self, index: usize) -> &Self::Output{
        match index{
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            3 => &self.r3,
            4 => &self.r4,
            5 => &self.r5,
            6 => &self.r6,
            7 => &self.r7,
            8 => &self.r8,
            9 => &self.r9,
            _ => panic!("Index out of bounds"),
        }
    }
}
impl IndexMut<usize> for Lab {
    fn index_mut(&mut self, index: usize) -> &mut Vec<char>{
        match index{
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            4 => &mut self.r4,
            5 => &mut self.r5,
            6 => &mut self.r6,
            7 => &mut self.r7,
            8 => &mut self.r8,
            9 => &mut self.r9,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Lab {
    fn move_up(&mut self, ) {
        let row_index = self.pos_x;
        let col_index = self.pos_y;
        if self[row_index-1][col_index] == '|' || self[row_index-1][col_index] == '_' {
            println!("You hit a wall");
        } else {
            let row = &mut self[row_index];
            row[col_index] = '.';
            let row_new = &mut self[row_index -1];
            row_new[col_index] = '?';
            self.pos_x -= 1;
        }
    } 
    fn move_right(&mut self) {
        let row_index = self.pos_x;
        let col_index = self.pos_y;
        if self[row_index][col_index+1] == '|' || self[row_index][col_index+1] == '_' {
            println!("You hit a wall");
        } else {
            let row = &mut self[row_index];
            row[col_index] = '.';
            row[col_index+1] = '?';
            self.pos_y += 1;
        }
    }
    fn move_left(&mut self) {
        let row_index = self.pos_x;
        let col_index = self.pos_y;
        if self[row_index][col_index-1] == '|' || self[row_index][col_index-1] == '_' {
            println!("You hit a wall");
        } else {
            let row = &mut self[row_index];
            row[col_index] = '.';
            row[col_index-1] = '?';
            self.pos_y -= 1;
        }
    }
    fn move_down(&mut self) {
        let row_index = self.pos_x;
        let col_index = self.pos_y;
        if self[row_index+1][col_index] == '|' || self[row_index+1][col_index] == '_' {
            println!("You hit a wall");
        } else {
            let row = &mut self[row_index];
            row[col_index] = '.';
            let row_new = &mut self[row_index+1];
            row_new[col_index] = '?';
            self.pos_x += 1;
        }
    }
    fn print_lab(&self) {
        let mut row_in_string = String::new();
        for i in 0..=9 {
            row_in_string = self[i].iter().collect();
            println!("{}",row_in_string)
        }

    }

}

fn create_lab() -> Lab{
    let mut labyrinth = Lab {
        r0: Vec::from(['_','_','_','_','_','_','_','_','_','_','_']),
        r1: Vec::from(['|','.','.','.','|','.','.','.','|','.','!']),
        r2: Vec::from(['|','.','|','.','|','.','|','.','|','.','|']),
        r3: Vec::from(['|','.','|','.','|','.','|','.','|','.','|']),
        r4: Vec::from(['|','.','|','.','|','.','|','.','|','.','|']),
        r5: Vec::from(['|','.','|','.','|','.','|','.','|','.','|']),
        r6: Vec::from(['|','.','|','.','|','.','|','.','|','.','|']),
        r7: Vec::from(['|','.','|','.','|','.','|','.','|','.','|']),
        r8: Vec::from(['|','?','|','.','.','.','|','.','.','.','|']),
        r9: Vec::from(['_','_','_','_','_','_','_','_','_','_','_']),
        pos_x: 8,
        pos_y: 1,
    };
    labyrinth
}

fn main() {
    println!("Hello world , welcome to lab game.");
    println!("Press W to go up, S to go down, A to go left or D to go right, or Q to quit");
    
    //let mut stdout = stdout().into_raw_mode().unwrap();
    let mut game_finished = false;
    let mut labyrinth = create_lab();
    
    while !game_finished {
        let mut user_input = String::new();
        labyrinth.print_lab();
        io::stdin().read_line(&mut user_input).expect("failed to read the line");
        match &user_input.len() {
            0|1 => {
                println!("You entered an empty string ");
                continue
            },
            2 => println!("Your character is {}", user_input),
            _ => {
                println!("You enter too many characters");
                continue
            }
        }
        labyrinth.print_lab();
        let key = user_input.chars().next().unwrap();
        
        match key{
            'w' => labyrinth.move_up(),
            's' => labyrinth.move_down(),
            'a' => labyrinth.move_left(),
            'd' => labyrinth.move_right(),
            'q' => break,
            _=>println!("Please input W,S,A OR D for movement"),
        }


        if labyrinth.pos_x == 1 && labyrinth.pos_y == 10 {
            
            println!("You win!");
            game_finished = true
        }
    }
    
}
