struct Game {
    current_piece : Piece,
    board : Vec<i32>,
}
impl Game {
    fn init() -> Game {
        Game {
            board : vec![0;250], 
            current_piece : Piece::new(),
        }
    }
    fn render(&self) {
        for yy in 0..25 {
            for xx in 0..10 {
                let mut ch = self.board[(yy*10) + xx];
                for i in 0..4 {
                    let wy = self.current_piece.y[i] as usize;
                    let wx = self.current_piece.x[i] as usize;
                    if xx == wx && yy == wy {
                        ch = self.current_piece.get_color();
                    }
                }
                print!("{:?}",ch);
            }
            println!();
        }
    }
    fn update(&mut self){

    }
    fn copy_piece_to_board(&mut self){
        
    }
   fn move_left(&mut self) -> bool {
        for i in 0..4 {
            let yy = self.current_piece.y[i] as usize;
            let xx = self.current_piece.x[i] as usize;
            if xx - 1 <= 0 {
                return false;
            }
            if self.board[(yy * 10) + xx - 1] != 0{
                return false; 
            }
        }
        for i in 0..4 {
            self.current_piece.x[i] = self.current_piece.x[i] - 1;
        }
        return true;
    }
    fn move_right(&mut self) -> bool {
        for i in 0..4 {
            let yy = self.current_piece.y[i] as usize;
            let xx = self.current_piece.x[i] as usize;
            if xx + 1 >= 10 {
                return false;
            }
            if self.board[(yy * 10) + xx + 1] != 0{
                return false; 
            }
        }
        for i in 0..4 {
            self.current_piece.x[i] = self.current_piece.x[i] + 1;
        }
        return true;
    }
    fn move_down(&mut self) -> bool {
        for i in 0..4 {
            let yy = self.current_piece.y[i] as usize;
            let xx = self.current_piece.x[i] as usize;
            if yy + 1 >= 25 {
                return false;
            }
            if self.board[(yy * 10 + 1) + xx] != 0{
                return false; 
            }
        }
        for i in 0..4 {
            self.current_piece.y[i] = self.current_piece.y[i] + 1;
        }
        return true;
    }
    fn rotate(&mut self) -> bool {
        match self.current_piece.color {
            Color::Red => {

            },
            Color::Yellow => {

            },
            Color::Magenta => {

            },
            Color::Cyan => {

            },
            Color::Blue => {

            },
            Color::Green => {

            },
            Color::Purple => {

            },
            _ => return false,
        }
        return false;
    }
}
enum Move {
    Left,
    Right,
    Down,
    Slam,
    Rotate,
}
enum Color {
    Red,
    Yellow,
    Magenta,
    Cyan,
    Blue,
    Green,
    Purple,
}
struct Piece {
    color : Color,    
    orientation : i32,
    x : Vec<i32>,
    y : Vec<i32>,
}
impl Piece {
    fn new() -> Piece {
        let oorientation = 7;
        let ccolor = match oorientation {
            1 => Color::Red,
            2 => Color::Yellow,
            3 => Color::Magenta,
            4 => Color::Cyan,
            5 => Color::Blue,
            6 => Color::Green,
            7 => Color::Purple,
            _ => Color::Yellow,
        };

        let xvec = match ccolor {
            Color::Red => vec![4,5,5,6],
            Color::Yellow => vec![5,6,5,6],
            Color::Magenta => vec![4,4,5,6],
            Color::Cyan => vec![5,5,5,5],
            Color::Blue => vec![4,5,6,6],
            Color::Green => vec![4,5,5,6],
            Color::Purple => vec![4,5,5,6],
            _ => vec![5,6,5,6],
        };
        let yvec = match ccolor {
            Color::Red => vec![3,3,4,4],
            Color::Yellow => vec![3,3,4,4],
            Color::Magenta => vec![3,4,3,3],
            Color::Cyan => vec![1,2,3,4],
            Color::Blue => vec![3,3,3,4],
            Color::Green => vec![4,4,3,3],
            Color::Purple => vec![4,4,3,4],
            _ => vec![3,3,4,4] ,
        };

        return Piece 
        {
            orientation : oorientation,
            color : ccolor,
            x : xvec,
            y : yvec,
        }
    }
    fn get_color(&self) -> i32 {
        match self.color {
            Color::Red => 1,
            Color::Yellow => 2,
            Color::Magenta => 3,
            Color::Cyan => 4,
            Color::Blue => 5,
            Color::Green => 6,
            Color::Purple => 7,
            _ => 0,
        }
    }
 }

fn main() {
    let mut game = Game::init();
    game.render();

}
