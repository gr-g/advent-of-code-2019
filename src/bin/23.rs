use advent_of_code_2019::intcode::IntCode;

fn solve(input: &str) -> (i64, i64) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut computers: Vec<_> = (0..50)
        .map(|i| {
            let mut c = IntCode::new(&program);
            c.input.push_back(i as i64);
            c.run();
            c
        })
        .collect();
    let mut nat = (0, 0);
    let mut first_nat_received = None;
    let mut last_nat_sent = None;
    let mut idle;

    loop {
        idle = true;
        for i in 0..50 {
            // process input received by computer i
            if computers[i].input.is_empty() {
                computers[i].input.push_back(-1);
            }
            computers[i].run();
            // dispatch packets from computer i
            let u: Vec<_> = computers[i].output.drain(..).collect();
            for pkt in u.chunks(3) {
                idle = false;
                let dest = pkt[0] as usize;
                if dest == 255 {
                    nat = (pkt[1], pkt[2]);
                    if first_nat_received.is_none() {
                        first_nat_received = Some(nat)
                    }
                } else {
                    computers[dest].input.push_back(pkt[1]);
                    computers[dest].input.push_back(pkt[2]);
                }
            }
        }
        if idle {
            // send nat packet to computer 0
            computers[0].input.push_back(nat.0);
            computers[0].input.push_back(nat.1);
            if last_nat_sent.filter(|(_, y)| *y == nat.1).is_some() {
                break;
            }
            last_nat_sent = Some(nat);
        }
    }

    (first_nat_received.unwrap().1, last_nat_sent.unwrap().1)
}

fn main() {
    let input = std::fs::read_to_string("input/23.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}
