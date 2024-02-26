mod event;
mod app;
mod game;
mod utils;

use std::{
    error::Error, 
    io,
    // thread,
    time::Duration
};

#[allow(unused_imports)]
use termion::{
    raw::IntoRawMode, 
    screen::AlternateScreen, 
    input::MouseTerminal,
    event::Key,
};

use tui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{
        Layout,
        Constraint,
        Direction
    },
    widgets::{
        canvas::{
            Canvas,
            Line,
            Points,
        },
        Block,
        Borders,
    },
    style::{
        Color, 
        Style
    },
    text::{
        Span,
        Spans,
    }
};

use crossterm::{
    event::{
        EnableMouseCapture,
        DisableMouseCapture,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use event::{
    Config,
    Events,
    Event,
};

use app::App;

use game::Command;

fn main() -> Result<(), Box<dyn Error>>
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    /*
        · execute!: 用于执行一系列终端操作的宏，可以帮助我在终端中执行多个操作
        · EnterAlternateScreen: 将终端切换到备用屏幕模式
        · EnableMouseCapture: 启用了鼠标捕获功能
     */
    execute!(
        stdout, 
        EnterAlternateScreen, 
        EnableMouseCapture
    )?;
    // let stdout = MouseTerminal::from(stdout);
    // let stdout = AlternateScreen::from(stdout);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };
    
    let events = Events::with_config(config);

    let mut app = App::new();

    // thread::sleep(Duration::from_millis(5000));
    loop 
    {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());

            let board_size = app.get_size();
            let panel_size = board_size + (board_size / 3.0);
            let half_box_size = app.box_size / 2.0;
            let font_width = 2.0;

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("2048-@yinxun"))
                .paint(|ctx| {
                    let grid = app.get_grid();
                    for (row, list) in grid.iter().enumerate()
                    {
                        for (col, _) in list.iter().enumerate()
                        {
                            let score = grid[row][col];
                            let s = if score == 0 {
                                String::from("").into_boxed_str()
                            } else {
                                pad_str(score.to_owned().to_string(), 6).into_boxed_str()
                            };

                            let x_box = (col as f64) * app.box_size;
                            let y_box = (row as f64) * app.box_size;

                            ctx.print(
                                ((col + 1) as f64) * app.box_size - half_box_size - font_width,
                                ((4 - row) as f64) * app.box_size - half_box_size - font_width * 2.0,
                                Spans::from(
                                    Span::styled(Box::leak(s).to_string(), Style::default().fg(score_to_color(score)))
                                ),
                            );
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box,
                                x2: x_box + app.box_size,
                                y2: y_box,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box,
                                x2: x_box,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box + app.box_size,
                                y1: y_box,
                                x2: x_box + app.box_size,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box + app.box_size,
                                x2: x_box + app.box_size,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                        }
                    }
                    if !app.is_alive()
                    {
                        ctx.draw(&Points {
                            coords: &app.get_game_over_modal(),
                            color: Color::Green
                        });

                        ctx.print(
                            app.box_size * 1.5,
                            app.box_size * 2.0,
                            Spans::from(
                                Span::styled(" GAME OVER! ", Style::default().fg(Color::Blue))
                            ),
                        );

                        ctx.print(
                            app.box_size * 1.3,
                            app.box_size * 1.8,
                            Spans::from(
                                Span::styled(" Restart[R] Quit[Q] ", Style::default().fg(Color::Blue))
                            ),
                        );
                    }
                })
                .x_bounds([0.0, board_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[0]);


            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Panel"))
                .paint(|ctx| {
                    ctx.print(
                        board_size, 
                        board_size, 
                        Spans::from(
                            Span::styled("> Relax <", Style::default().fg(Color::Blue))
                        ),
                    );

                    let score = app.get_score().to_owned().to_string().into_boxed_str();
                    ctx.print(board_size, board_size - 15.0, "Score: ");
                    ctx.print(
                        board_size + 10.0,
                        board_size - 20.0,
                        Spans::from(
                            Span::styled(Box::leak(score).to_string(), Style::default().fg(Color::Green))
                        ),
                    );
                    ctx.print(board_size, 0.0, "Quit[Q]");
                })
                .x_bounds([board_size, panel_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => { break; }
                Key::Char('r') => { app.restart(); }
                Key::Down => { app.add_command(Command::Down); }
                Key::Up => { app.add_command(Command::Up); }
                Key::Right => { app.add_command(Command::Right); }
                Key::Left => { app.add_command(Command::Left); }
                Key::Char('h') => { app.add_command(Command::Left); }
                Key::Char('k') => { app.add_command(Command::Down); }
                Key::Char('l') => { app.add_command(Command::Right); }
                Key::Char('j') => { app.add_command(Command::Up); }
                _ => { app.add_command(Command::Nil); }
            },
            Event::Tick => { app.next() }
        }
    }


    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    terminal.show_cursor()?;

    Ok(())
}

fn pad_str(s: String, length: usize) -> String
{
    let mut s = s.clone();

    loop 
    {
        if s.len() < length 
        {
            s.push_str(" ");
        } else 
        {
            break;
        }
    }
    s
}

fn score_to_color(score: i32) -> Color 
{
    if score < 64 { Color::Green }
    else if score < 256 { Color::Magenta }
    else if score < 1024 { Color::Cyan }
    else if score < 4096 { Color::LightRed }
    else { Color::Red }
}