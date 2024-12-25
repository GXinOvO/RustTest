
mod tests;

pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let L = words.len();
    let max_width = max_width as usize;
    let mut queue = Vec::new();

    let mut i = 0;
    let mut count = 0;
    let mut tmp = Vec::new();

    while i < L {
        count = 0;
        tmp = Vec::new();

        while i < L && count < max_width {
            if count + words[i].len() > max_width {
                break;
            }

            count += words[i].len() + 1;
            tmp.push(words[i].clone());
            i += 1;
        }

        if i == L {
            let last_line = tmp[..].join(&" ");
            let len = last_line.len();
            queue.push(last_line + &" ".repeat(max_width - len));
            break;
        }

        if tmp.len() == 1 {
            queue.push(tmp[..].join(&"") + &" ".repeat(max_width - count + 1));
        } else {
            let T = tmp.len() - 1;
            let paddings = max_width - (count - T - 1);
            let space = paddings / T;
            let mut extra = (paddings % T) as i32;
            let res = tmp.into_iter()
                .enumerate()
                .map(|(i, x)| {
                    let times = space + if extra <= 0 { 0 } else { 1 };
                    extra -= 1;

                    if i == T {
                        x
                    } else {
                        x + &" ".repeat(times)
                    }
                })
                .collect::<String>();
            queue.push(res);
        }
    }
    queue
}

fn main() {
    println!("Hello, world!");
}
