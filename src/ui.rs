use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph, Frame
};
use std::io::{stdout, Result};

use self::keybinds::{default_keybinds, Action};

mod keybinds;

pub fn draw_ui() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let keybinds = default_keybinds();
    
    // main loop
    loop {
        terminal.draw(draw_paragraph)?;
        
        let key = get_key().unwrap();
        
        // todo: make this less ugly?
        match key {
            Some(k) => {
                let action = keybinds.get(&k);

                match action {
                    Some(a) => {
                        match a {
                            Action::Call(f) => f(),
                            Action::Quit => break
                        }
                    }, 
                    None => () // do nothing if there is no action for pressed key
                }
            },
            None => () // do nothing if no key is pressed
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    
    Ok(())
}

fn get_key() -> Result<Option<KeyCode>> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
            {
                return Ok(Some(key.code))
            }
        }
    }
    Ok(None)
}

fn pause() {

}

fn draw_paragraph(frame: &mut Frame) {
    frame.render_widget(Paragraph::new("Hello, World!")
        .white()
        .on_blue(),
        frame.size()
    )
}