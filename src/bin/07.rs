use advent_of_code_2019::intcode::IntCode;

trait Permute {
    fn permute(&mut self, n: usize);
}

impl<T> Permute for [T] {
    // Permutes a slice of length N in one of the N! possible ways.
    // n = 0 corresponds to the identity permutation.
    // panics if n >= N! or N! > usize::MAX
    fn permute(&mut self, mut n: usize) {
        let mut l = self.len();
        for pos in 0..l-1 {
            self.swap(pos, pos + (n % l));
            n /= l;
            l -= 1;
        }
        assert_eq!(n, 0);
    }
}

fn run_amplifiers( program: &[i64], config: &[i64; 5] ) -> i64 {
    let mut amp = [IntCode::new(program), IntCode::new(program),
        IntCode::new(program), IntCode::new(program), IntCode::new(program)];
        
    for i in 0..5 {
        amp[i].input.push_back(config[i]);
        amp[i].run();
    }

    amp[0].input.push_back(0);
    loop {
        amp[0].run();
        amp[1].input.push_back(amp[0].output.pop().unwrap());
        amp[1].run();
        amp[2].input.push_back(amp[1].output.pop().unwrap());
        amp[2].run();
        amp[3].input.push_back(amp[2].output.pop().unwrap());
        amp[3].run();
        amp[4].input.push_back(amp[3].output.pop().unwrap());
        amp[4].run();
        if amp[4].is_halted() { break; }
        amp[0].input.push_back(amp[4].output.pop().unwrap());
    }
    amp[4].output.pop().unwrap()
}

fn max_amplifiers( program: &[i64], range: &[i64; 5] ) -> ([i64; 5], i64) {
    let mut max = ([0; 5], 0);
    
    for i in 0..120 {
        let mut config = *range;
        config.permute(i);
        let res = run_amplifiers(program, &config);
        if res > max.1 {
            max = (config, res);
        }
    }
    max
}

fn solve( input: &str ) -> (i64, i64) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();

    let highest_signal = max_amplifiers(&program, &[0, 1, 2, 3, 4]).1;
    let highest_signal_with_feedback = max_amplifiers(&program, &[5, 6, 7, 8, 9]).1;
    
    (highest_signal, highest_signal_with_feedback)
}

fn main() {
    let input = std::fs::read_to_string("input/07.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example01() {
        assert_eq!(max_amplifiers(&[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], &[0,1,2,3,4]), ([4,3,2,1,0], 43210));
    }

    #[test]
    fn example02() {
        assert_eq!(max_amplifiers(&[3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], &[0,1,2,3,4]), ([0,1,2,3,4], 54321));
    }

    #[test]
    fn example03() {
        assert_eq!(max_amplifiers(&[3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], &[0,1,2,3,4]), ([1,0,4,3,2], 65210));
    }

    #[test]
    fn example04() {
        assert_eq!(max_amplifiers(&[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5], &[5,6,7,8,9]), ([9,8,7,6,5], 139629729));
    }

    #[test]
    fn example05() {
        assert_eq!(max_amplifiers(&[3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10], &[5,6,7,8,9]), ([9,7,8,5,6], 18216));
    }
}
