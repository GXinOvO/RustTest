mod app;
mod event;
mod game;
mod utils;

use std::{error::Error, io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color},
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders,
    },
    Terminal,
};

use app::App;
use event::{Config, Event, Events};
use game::Command;

/*
    Result<T, E>: 这是一个泛型类型，其中T代表成功时的返回值的类型，E代表失败时的错误类型。
    Box<dyn Error>表示一个堆分配的Error对象的指针，dyn Error表示类型擦除(type erasure)，允许在运行时处理不同类型的错误。
*/
fn main() -> Result<(), Box<dyn Error>> {

    /*
        · 获取标准输出流并将其设置为原始模式。
            io::stdout(): 返回一个表示标准输出的类型
            into_raw_mode(): 将该类型转换为原始模式 

        · 创建一个MouseTerminal对象，用于处理鼠标输入
        · 将终端切换到备用屏幕。这样做是为了能够在程序运行期间绘制TUI而不影响终端的原始内容。
        · 创建一个TermionBackend对象，是一个终端后端，他将用于与TUI进行交互
        · 创建Terminal对象，将用于在终端上绘制TUI。
     */
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // -> 用于配置时间生成器的行为。在这里，tick_rate字段被设置为每250毫秒生成一个事件。
    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };

    // -> 用于处理终端时间。
    let events = Events::with_config(config);

    // -> App时应用程序的主要逻辑部分。
    let mut app = App::new();

    /*
    --TODO:
        不断地在终端上绘制游戏界面并处理用户输入事件。
     */
    loop {
        // -> terminal.draw()绘制游戏界面。
        terminal.draw(|f| {
            /*
                · 使用TUI库中的Layout类型来创建一个水平方向的布局，并定义了两个约束条件:
                    80%的宽度和20%的宽度。
                    然后使用.split(f.size())将父容器的大小分割为两个子容器的大小，并将结果存储在chunks变量中。
             */
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            // params
            /*
                · 游戏板的大小
                · 面板的大小(在游戏板大小的基础上增加1/3)
                · 盒子大小的一半
                · 字体的宽度
             */
            let board_size = app.get_size();
            let panel_size = board_size + (board_size / 3.0);
            let half_box_size = app.box_size / 2.0;
            let font_width = 2.0;
            // Game board
            /*
            --TODO:
                用于绘制游戏板。

                Canvas对象包含一个块(Block),块具有边框和标题。
                paint()接收一个闭包，用于在画布上绘制游戏板。
                x_bounds()和y_bounds()用于设置画布的水平和垂直边界。
                最后，使用f.render_widget()将画布渲染到第一个子容器(chunks[0])
             */
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("2048-@wander"))
                .paint(|ctx| {
                    /*  
                        获取游戏板的二维数组，然后使用enumerate()对其进行遍历。
                    在外层，enumerate()返回第一行索引row和对应的列表list。
                    在内层，enumerate()返回第一列的索引和对应的元素。(这里用下划线表示，因此不使用该值)
                    */
                    let grid = app.get_grid();
                    for (row, list) in grid.iter().enumerate() {
                        for (col, _) in list.iter().enumerate() {
                            // 盒子参数
                            // -> 盒子分数score创建要显示的字符串s。如果分数为0，那么字符串为空；否则，将使用pad_str()函数将分数转换为6位字符串，并将结果转换位Box<str>
                            let score = grid[row][col];
                            let s = if score == 0 {
                                String::from("").into_boxed_str()
                            } else {
                                pad_str(score.to_owned().to_string(), 6).into_boxed_str()
                            };
                            // -> 计算当前盒子的左上角坐标，乘以app.box_size可以确定盒子的位置。
                            let x_box = (col as f64) * app.box_size;
                            let y_box = (row as f64) * app.box_size;
                            // -> 在画布上打印盒子的分数。通过调整坐标，使得分数居中显示。score_to_color()用于根据分数确定颜色
                            ctx.print(
                                ((col + 1) as f64) * app.box_size - half_box_size - font_width,
                                ((4 - row) as f64) * app.box_size
                                    - half_box_size
                                    - font_width * 2.0,
                                Box::leak(s),
                                score_to_color(score),
                            );
                            // -> 绘制盒子的边界。通过创建四条线段，分别连接盒子的四个角，从而形成盒子的边界。每条线段都具有起始点和终止点的坐标，以及指定的颜色。
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

                    /*
                    --TODO:
                        在游戏结束时绘制游戏结束的提示信息。
                            
                        使用app.is_alive()检查游戏是否结束。如果游戏结束，那么使用ctx.draw()方法绘制一个点，
                    点的坐标由app.get_game_over_model()返回.
                    
                     */
                    if !app.is_alive() {

                        ctx.draw(&Points {
                            coords: &app.get_game_over_modal(),
                            color: Color::Green
                        });

                        ctx.print(
                            app.box_size * 1.5,
                            app.box_size * 2.0,
                            " GAME OVER! ",
                            Color::Blue,
                        );

                        ctx.print(
                            app.box_size * 1.3,
                            app.box_size * 1.8,
                            " Restart[R] Quit[Q] ",
                            Color::Blue,
                        );
                    }
                })
                .x_bounds([0.0, board_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[0]);
            // Informantions
            /*
            --TODO:
                用于绘制信息面板。
             */
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Panel"))
                .paint(|ctx| {
                    ctx.print(board_size, board_size, "> Relax <", Color::Blue);

                    let score = app.get_score().to_owned().to_string().into_boxed_str();
                    ctx.print(board_size, board_size - 30.0, "Score:", Color::Green);
                    ctx.print(
                        board_size,
                        board_size - 40.0,
                        Box::leak(score),
                        Color::Green,
                    );

                    ctx.print(board_size, 0.0, "Quit[Q]", Color::Blue);
                })
                .x_bounds([board_size, panel_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[1]);
        })?;

        // Events
        /*
        --TODO:
            获取下一个事情，并根据时间的类型进行处理。
         */
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('r') => {
                    app.restart();
                }
                // left up right down
                Key::Down => {
                    app.add_command(Command::Down);
                }
                Key::Up => {
                    app.add_command(Command::Up);
                }
                Key::Right => {
                    app.add_command(Command::Right);
                }
                Key::Left => {
                    app.add_command(Command::Left);
                }
                // h k l j   vim keys support
                Key::Char('h') => {
                    app.add_command(Command::Left);
                }
                Key::Char('k') => {
                    app.add_command(Command::Up);
                }
                Key::Char('l') => {
                    app.add_command(Command::Right);
                }
                Key::Char('j') => {
                    app.add_command(Command::Down);
                }
                _ => {
                    app.add_command(Command::Nil);
                }
            },
            Event::Tick => {
                app.next()
            }
        }
    }

    Ok(())
}

/// make different strings as same length
fn pad_str(s: String, length: usize) -> String {
    let mut s = s.clone();
    loop {
        if s.len() < length {
            s.push_str(" ");
        } else {
            break;
        }
    }

    s
}

/// render different color for different score
fn score_to_color(score: i32) -> Color {
    if score < 64 {
        Color::Green
    } else if score < 256 {
        Color::Magenta
    } else if score < 1024 {
        Color::Cyan
    } else if score < 4096 {
        Color::LightRed
    } else {
        Color::Red
    }
}
