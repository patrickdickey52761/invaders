use std::error::Error;
use crossterm::cursor::{Hide, Show};
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use std::{io, thread};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::event::{Event, KeyCode};
use invaders::{frame, render};
use invaders::frame::{Drawable, new_frame};
use invaders::invaders::Invaders;
use invaders::player::Player;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "audio/explode.wav");
    audio.add("lose","audio/lose.wav");
    audio.add("move","audio/move.wav");
    audio.add("pew","audio/pew.wav");
    audio.add("startup","audio/startup.wav");
    audio.add("win","audio/win.wav");
    audio.play("startup");

    //Terminal init
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            }; //end match
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame= curr_frame;
        } //end loop
    }); //end thread::spawn

    //Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        //Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }//end if player.shoot
                    },//end KeyCode Space or Enter -- SHoot
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    } //end quit KeyCodes
                    _ => {}
                } //end match
            } //endif
        } //end while

        //Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }//endif invaders.update
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }//end if player hit something
        //Draw & render
        //player.draw(&mut curr_frame);
        //invaders.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }//end for replacing two commented lines above
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        //Win or Lose

        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }//end if all killed
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    } //end 'gameloop

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
} //end main()
