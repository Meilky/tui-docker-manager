extern crate rs_docker;
use rs_docker::Docker;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Error},
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod statefull_list;
mod ui;
use statefull_list::StatefullList;
mod store;

fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
    mut list: StatefullList<String>,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, &mut list))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => list.unselect(),
                    KeyCode::Down => list.next(),
                    KeyCode::Up => list.previous(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn callback() {
    println!("{}", "allo");
}

fn main() -> Result<(), Error> {
    let mut docker = Docker::connect("unix:///var/run/docker.sock")?;
    let containers = docker.get_containers(false)?;

    let mut terminal = init_terminal()?;

    let tick_rate = Duration::from_millis(250);
    let mut items: Vec<String> = vec![];

    for c in containers {
        items.push(c.Names.join(", "));
    }

    let list = StatefullList::with_items(items);

    let res = run_tui(&mut terminal, tick_rate, list);

    reset_terminal()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

/// Initializes the terminal.
fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Error> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

/// Resets the terminal.
fn reset_terminal() -> Result<(), Error> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
