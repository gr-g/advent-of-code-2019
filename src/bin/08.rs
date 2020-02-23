struct SpaceImage {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl SpaceImage {
    fn count012( &self ) -> Vec<(usize, usize, usize)> {
        self.data.chunks_exact(self.width*self.height).map(|l| {
            l.iter().fold((0,0,0), |(n0,n1,n2), d| {
                match d {
                    b'0' => (n0+1,n1,n2),
                    b'1' => (n0,n1+1,n2),
                    b'2' => (n0,n1,n2+1),
                    _ => (n0,n1,n2),
                }
            })
        }).collect()
    }

    fn compose_layers( &self ) -> Vec<u8> {
        let mut res: Vec<u8> = vec![b'0'; self.width*self.height];
        
        for i in 0..self.width*self.height {
            res[i] = *self.data.iter().skip(i)
                .step_by(self.width*self.height)
                .find(|&d| d != &b'2')
                .unwrap_or(&b'0');
        }
        res
    }
   
    fn decode( &self ) -> String {
        let mut it = self.compose_layers().into_iter();
        let mut s = String::new();
        for _ in 0..self.height {
            for _ in 0..self.width {
                if it.next().unwrap() == b'1' {
                    s.push('\u{2588}')
                } else {
                    s.push(' ')
                };
            }
            s.push('\n');
        }
        s
    }
}

fn solve( input: &str, width: usize, height: usize ) -> (usize, String) {
    let img = SpaceImage{ width, height, data: input.as_bytes().to_vec() };
    
    let (_, c1, c2) = img.count012().into_iter()
        .min_by(|&(a0, _, _),&(b0, _, _)| usize::cmp(&a0, &b0))
        .unwrap();
    let min12 = c1 * c2;
    let decoded_img = img.decode();
    
    (min12, decoded_img)
}

fn main() {
    let input = std::fs::read_to_string("input/08.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input, 25, 6);
    println!("Solution: {}\n{}", s.0, s.1);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example01() {
        let img = SpaceImage{ width: 2, height: 2, data: b"0222112222120000".to_vec() };
        assert_eq!(img.compose_layers(), b"0110".to_vec());
    }
}
