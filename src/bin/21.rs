use advent_of_code_2019::intcode::IntCode;

fn solve(input: &str) -> (i64, i64) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();

    let mut droid = IntCode::new(&program);
    droid.run_ascii_command("");
    droid.run_ascii_command("OR A J"); // J = A
    droid.run_ascii_command("AND B J"); // J = (A && B)
    droid.run_ascii_command("AND C J"); // J = (A && B && C)
    droid.run_ascii_command("NOT J J"); // J = !(A && B && C)
    droid.run_ascii_command("AND D J"); // J = !(A && B && C) && D
    let (_, hull_damage) = droid.run_ascii_command("WALK");

    let mut droid = IntCode::new(&program);
    droid.run_ascii_command("");
    droid.run_ascii_command("OR A T"); // T = A
    droid.run_ascii_command("AND B T"); // T = (A && B)
    droid.run_ascii_command("AND C T"); // T = (A && B && C)
    droid.run_ascii_command("NOT T T"); // T = !(A && B && C)
    droid.run_ascii_command("OR I J"); // J = I
    droid.run_ascii_command("OR F J"); // J = (I || F)
    droid.run_ascii_command("AND E J"); // J = E && (I || F)
    droid.run_ascii_command("OR H J"); // J = H || (E && (I || F))
    droid.run_ascii_command("AND D J"); // J = D && (H || (E && (I || F)))
    droid.run_ascii_command("AND T J"); // J = !(A && B && C) && D && (H || (E && (I || F)))
    let (_, hull_damage_ext) = droid.run_ascii_command("RUN");

    (hull_damage[0], hull_damage_ext[0])
}

fn main() {
    let input = std::fs::read_to_string("input/21.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}
