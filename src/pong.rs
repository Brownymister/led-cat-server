use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::Error;

#[cfg(test)]
#[path = "./pong_test.rs"]
mod pong_test;

pub type GameBoard = Vec<Vec<Entity>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Game {
    pub board: GameBoard,
    pub running: bool,
    pub player_y_pos: i32,
    pub ball_pos: BallPos,
    pub player_x_pos: i32,
    pub counter: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BallPos {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Entity {
    Ball,
    Player,
    Empty,
}

impl Game {
    pub fn new() -> Self {
        let mut board = vec![];
        for y in 1..65 {
            let mut lane = vec![];
            for x in 1..65 {
                if y == 32 && x == 32 {
                    lane.push(Entity::Ball);
                } else if y == 32 && x == 64 {
                    lane.push(Entity::Player);
                } else {
                    lane.push(Entity::Empty);
                }
            }
            board.push(lane);
        }
        let mut rng = rand::thread_rng();
        return Game {
            board,
            running: false,
            // 0 -> 31 => 23 (!)
            player_y_pos: 31,
            player_x_pos: 64 - 3,
            ball_pos: BallPos {
                x: 31,
                y: 31,
                vx: rng.gen_range(-1..1),
                vy: rng.gen_range(-1..1),
            },
            counter: 0,
        };
    }

    pub fn json(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }

    pub fn display(&self) {
        print!("");
        // println!("----------------------------------------------------------------");
        // for y in self.board.iter().rev() {
        //     let mut line = "".to_string();
        //     for x in y {
        //         line = line
        //             + match x {
        //                 Entity::Player => "P",
        //                 Entity::Ball => "X",
        //                 _ => " ",
        //             }
        //     }
        //     line = line + "|";
        //     println!("{}", line);
        // }
        // println!("----------------------------------------------------------------");
    }

    // ball boucing algorython, see: https://www.101computing.net/bouncing-algorithm/
    pub fn move_ball(&mut self) {
        let old_pos = BallPos {
            x: self.ball_pos.x,
            y: self.ball_pos.y,
            vy: self.ball_pos.vy,
            vx: self.ball_pos.vx,
        };

        let new_pos_x = self.ball_pos.x + self.ball_pos.vx;
        let new_pos_y = self.ball_pos.y + self.ball_pos.vy;

        let mut rng = rand::thread_rng();

        let ball_crossed_player_barrier = new_pos_x >= 64;
        if ball_crossed_player_barrier || self.ball_pos.vx == 0 || self.ball_pos.vy == 0 {
            self.ball_pos.x = 31;
            self.ball_pos.y = 31;
            self.ball_pos.vx = rng.gen_range(-1..1);
            self.ball_pos.vy = rng.gen_range(-1..1);
            return;
        }

        self.ball_pos.x = new_pos_x;
        self.ball_pos.y = new_pos_y;

        if self.ball_pos.x < 0 {
            self.ball_pos.vx = -self.ball_pos.vx;
            self.ball_pos.x = self.ball_pos.x + self.ball_pos.vx;
        }

        if (self.ball_pos.x == self.player_x_pos
            && (self.ball_pos.y == self.player_y_pos
                || self.ball_pos.y == self.player_y_pos - 1
                || self.ball_pos.y == self.player_y_pos + 1))
        {
            self.ball_pos.vx = -self.ball_pos.vx;
            self.ball_pos.x = self.ball_pos.x + self.ball_pos.vx;
            self.counter += 1;
        }
        if self.ball_pos.y < 0 || self.ball_pos.y > 63 {
            self.ball_pos.vy = -self.ball_pos.vy;
            self.ball_pos.y = self.ball_pos.y + self.ball_pos.vy;
        }

        self.board[old_pos.x as usize][old_pos.y as usize] = Entity::Empty;
        self.board[self.ball_pos.x as usize][self.ball_pos.y as usize] = Entity::Ball;
        // self.board[self..x][old_pos.y] = Entity::Empty;
    }

    pub fn player_up(&mut self) -> Result<(), Error> {
        if self.player_y_pos > 2 {
            let x = 64 - 3;
            self.board[self.player_y_pos as usize][x] = Entity::Empty;
            self.board[(self.player_y_pos - 1) as usize][x] = Entity::Player;
            self.player_y_pos = self.player_y_pos - 1;
            return Ok(());
        }
        return Err(Error::new(std::io::ErrorKind::Other, "Out of range"));
    }

    pub fn player_down(&mut self) -> Result<(), Error> {
        if self.player_y_pos < 62 {
            let x = 64 - 3;
            self.board[self.player_y_pos as usize][x] = Entity::Empty;
            self.board[(self.player_y_pos + 1) as usize][x] = Entity::Player;
            self.player_y_pos = self.player_y_pos + 1;
            return Ok(());
        }
        return Err(Error::new(std::io::ErrorKind::Other, "Out of range"));
    }
}
