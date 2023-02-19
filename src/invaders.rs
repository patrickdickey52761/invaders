use std::cmp::max;
use std::time::Duration;
use rusty_time::timer::Timer;
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}//end single Invader

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}//end struct Invaders

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (x %2 == 0)
                    && (y %2 == 0)
                && x == 2 {
                    army.push(Invader {x, y});
                }//endif
            }//end for y
        }//end for x
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }//end return Self
    }//end new()
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }//end if min_x == 0
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS -1 {
                    self.direction = -1;
                    downwards = true;
                }//end max_x==NUM_COLS -1
            }//end direction
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis()- 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                } //end for
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32)+ self.direction) as usize;
                }//end for
            }//end if downwards
            return true;
        }//endif self.move_timer.ready
        false
    }//end update()
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }//end all_Killed()
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0)>= NUM_ROWS -1
    }//end reached_bottom
    pub fn kill_invader_at(&mut self, x:usize, y:usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y)) {
            self.army.remove(idx);
            true
        } else {
            false
        }//end if killed invader
    }//end kill_invader_at
}//end impl Invaders

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32()) > 0.5 {
                "x"
            } else {
                "+"
            };
        }//end for
    }//end draw()
}//end impl Drawable()