use crate::game::{Game, Grid, Command};

pub struct App 
{
    pub x: f64,
    pub y: f64,
    pub box_size: f64,
    game: Game,
    queue: Vec<Command>,
    score: i32,
}

impl App 
{
    pub fn new() -> App
    {
        let game = Game::new();
        let mut app = App {
            x: 0.0,
            y: 0.0,
            box_size: 40.0,
            game,
            queue: Vec::new(),
            score: 0
        };

        app.game.start();

        app        
    }

    pub fn get_size(&self) -> f64
    {
        self.box_size * 4.0
    }

    pub fn get_grid(&self) -> Grid 
    {
        self.game.get_grid()
    }

    pub fn get_score(&self) -> i32
    {
        self.score
    }

    pub fn is_alive(&self) -> bool 
    {
        self.game.alive
    }

    pub fn add_command(&mut self, cmd: Command)
    {
        if self.is_alive()
        {
            self.queue.insert(0, cmd)
        }
    }

    pub fn restart(&mut self) 
    {
        if !self.is_alive()
        {
            self.game = Game::new();
            self.game.start();
            self.queue = vec![];
            self.score = 0;
        }
    }

    pub fn next(&mut self)
    {
        if self.is_alive() && !self.queue.is_empty()
        {
            if let Some(top) = self.queue.pop()
            {
                if top != Command::Nil 
                {
                    self.game.next_tick(top);
                    self.score = self.game.get_score();
                }
            }
        }
    }

    pub fn get_game_over_modal(&self) -> Vec<(f64, f64)> 
    {
        let mut all: Vec<(f64, f64)> = vec![];
        let board_size = self.get_size();

        let x = board_size / 2.0 - self.box_size * 1.5;
        let y = board_size / 2.0 - self.box_size / 2.0;
        let width = self.box_size * 3.0;
        let height = self.box_size - 10.0;
        let mut p_x = x;
        loop {
            let i = p_x + 1.0;
            if i >= x + width { break; }
            let mut p_y = y;
            loop {
                let j = p_y + 1.0;
                if j >= y + height { break; }
                all.push((i, j));
                p_y = j;
            }
            p_x = i;
        }
        all
    }
}