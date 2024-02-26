use std::{
    io,
    time::Duration,
    sync::mpsc,
    thread,
};

use termion::{
    event::Key,
    input::TermRead,
};

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug, Clone, Copy)]
pub struct Config
{
    pub tick_rate: Duration,
}

impl Default for Config
{
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

#[allow(dead_code)]
pub struct Events 
{
    // -> mpsc是std库的多生产者单消费者通道，Receiver是通道的接收端，用于接收从其他线程发送的信息。
    //      rx用于接收Event<Key>类型的消息
    rx: mpsc::Receiver<Event<Key>>,
    // -> JoinHandle是std库中的线程句柄类型，用于控制线程的执行和等待线程结束。
    //      input_handle用于存储与键盘事件相关的线程句柄。  
    input_handle: thread::JoinHandle<()>,
    // -> tick_handle用于存储与每个时钟周期相关的线程句柄。
    tick_handle: thread::JoinHandle<()>,
}

impl Events 
{
    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();

        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                // -> 获取标准输入流的句柄，用于读取用户的键盘输入。
                let stdin = io::stdin();
                for evt in stdin.keys()
                {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key))
                        {
                            eprintln!("input_handle: {}", err);
                            return ;
                        }
                    }
                }
            })
        };
        
        let tick_handle = {
            thread::spawn(move || loop {
                if let Err(err) = tx.send(Event::Tick) {
                    eprintln!("tick_handle: {}", err);
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };

        Events {
            rx, 
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError>
    {
        self.rx.recv()
    }
}