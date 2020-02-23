use std::collections::HashMap;

struct ReactionItem {
    amount: i64,
    item: String,
}

impl ReactionItem {
    fn create_from(s: &str) -> ReactionItem {
        let mut el = s.split(' ');
        ReactionItem {
            amount: el.next().unwrap().parse::<i64>().unwrap(),
            item: el.next().unwrap().to_string(),
        }
    }
}

struct Reaction {
    output: ReactionItem,
    inputs: Vec<ReactionItem>,
}

fn read_formulas(input: &str) -> HashMap<String, Reaction> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(" => ");
            let inputs = it
                .next()
                .unwrap()
                .split(", ")
                .map(ReactionItem::create_from)
                .collect();
            let output = ReactionItem::create_from(it.next().unwrap());
            (output.item.clone(), Reaction { output, inputs })
        })
        .collect()
}

fn ore_for_fuel(formulas: &HashMap<String, Reaction>, fuel: i64) -> i64 {
    let mut required_items = HashMap::new();
    required_items.insert("FUEL".to_string(), fuel);

    while let Some((required_item, required_amount)) = required_items
        .iter_mut()
        .find(|(item, amount)| *item != "ORE" && **amount > 0)
    {
        let reaction = formulas.get(required_item).unwrap();

        let n = (*required_amount - 1) / reaction.output.amount + 1;
        *required_amount -= reaction.output.amount * n;
        for input in reaction.inputs.iter() {
            let item_amount = required_items.entry(input.item.clone()).or_insert(0);
            *item_amount += input.amount * n;
        }
    }
    required_items["ORE"]
}

fn solve(input: &str, target: i64) -> (i64, i64) {
    let formulas = read_formulas(input);
    let ore_for_1 = ore_for_fuel(&formulas, 1);

    let mut est_fuel = target / ore_for_1;

    loop {
        let ore = ore_for_fuel(&formulas, est_fuel);
        if ore > target {
            break;
        }
        est_fuel += std::cmp::max(1, (target - ore) / ore_for_1);
    }

    (ore_for_1, est_fuel - 1)
}

fn main() {
    let input = std::fs::read_to_string("input/14.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input, 1_000_000_000_000);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(ore_for_fuel(&read_formulas("\
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"), 1), 31);
    }

    #[test]
    fn example02() {
        assert_eq!(ore_for_fuel(&read_formulas("\
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"), 1), 165);
    }

    #[test]
    fn example03() {
        assert_eq!(solve("\
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT", 1_000_000_000_000), (13312, 82892753));
    }

    #[test]
    fn example04() {
        assert_eq!(solve("\
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF", 1_000_000_000_000), (180697, 5586022));
    }

    #[test]
    fn example05() {
        assert_eq!(solve("\
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX", 1_000_000_000_000), (2210736, 460664));
    }
}
