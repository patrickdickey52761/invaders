use rusty_time::timer::Timer;
use std::time::Duration;
use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
} //end struct Shot
 impl Shot {
    pub fn new (x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }//end Self
    }//end new()
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }//end move
            self.timer.reset();
        }//end if self.timer.ready or exploding
    }//end update()
    pub fn explode (&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    } //end explode
    pub fn dead (&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }//end dead

} //end impl Shot()

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }//end draw()
}//end impl Drawable