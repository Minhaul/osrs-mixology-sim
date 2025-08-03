use std::{cmp::max, collections::HashMap};

use enum_iterator::{Sequence, all};
use rand::prelude::*;
use rand_distr::weighted::WeightedAliasIndex;

// Number of orders to simulate
const NUM_ORDERS_TO_SIM: usize = 10_000_000;

// Potions in order of value based on unique reward prices
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Sequence, Debug)]
enum MixologyPotion {
    AAA,
    MMM,
    LLL,
    AAM,
    AAL,
    MMA,
    MML,
    LLA,
    LLM,
    MAL,
}

impl MixologyPotion {
    #[inline]
    fn input_points(&self) -> MixologyPoints {
        match self {
            MixologyPotion::MMM => MixologyPoints { M: 30, A: 0, L: 0 },
            MixologyPotion::AAA => MixologyPoints { M: 0, A: 30, L: 0 },
            MixologyPotion::LLL => MixologyPoints { M: 0, A: 0, L: 30 },
            MixologyPotion::MMA => MixologyPoints { M: 20, A: 10, L: 0 },
            MixologyPotion::MML => MixologyPoints { M: 20, A: 0, L: 10 },
            MixologyPotion::AAM => MixologyPoints { M: 10, A: 20, L: 0 },
            MixologyPotion::AAL => MixologyPoints { M: 0, A: 20, L: 10 },
            MixologyPotion::LLM => MixologyPoints { M: 10, A: 0, L: 20 },
            MixologyPotion::LLA => MixologyPoints { M: 0, A: 10, L: 20 },
            MixologyPotion::MAL => MixologyPoints { M: 10, A: 10, L: 10 },
        }
    }

    #[inline]
    fn output_points(&self) -> MixologyPoints {
        match self {
            MixologyPotion::MMM => MixologyPoints { M: 20, A: 0, L: 0 },
            MixologyPotion::AAA => MixologyPoints { M: 0, A: 20, L: 0 },
            MixologyPotion::LLL => MixologyPoints { M: 0, A: 0, L: 20 },
            MixologyPotion::MMA => MixologyPoints { M: 20, A: 10, L: 0 },
            MixologyPotion::MML => MixologyPoints { M: 20, A: 0, L: 10 },
            MixologyPotion::AAM => MixologyPoints { M: 10, A: 20, L: 0 },
            MixologyPotion::AAL => MixologyPoints { M: 0, A: 20, L: 10 },
            MixologyPotion::LLM => MixologyPoints { M: 10, A: 0, L: 20 },
            MixologyPotion::LLA => MixologyPoints { M: 0, A: 10, L: 20 },
            MixologyPotion::MAL => MixologyPoints { M: 20, A: 20, L: 20 },
        }
    }

    #[inline]
    fn weight(&self) -> u8 {
        match self {
            MixologyPotion::MMM => 5,
            MixologyPotion::AAA => 5,
            MixologyPotion::LLL => 5,
            MixologyPotion::MMA => 4,
            MixologyPotion::MML => 4,
            MixologyPotion::AAM => 4,
            MixologyPotion::AAL => 4,
            MixologyPotion::LLM => 4,
            MixologyPotion::LLA => 4,
            MixologyPotion::MAL => 3,
        }
    }
}

struct MixologyOrder {
    first: MixologyPotion,
    second: MixologyPotion,
    third: MixologyPotion,
}

impl MixologyOrder {
    fn contains(&self, pot: MixologyPotion) -> bool {
        self.first == pot || self.second == pot || self.third == pot
    }

    fn num(&self, pot: MixologyPotion) -> u8 {
        let mut num = 0;
        if self.first == pot {
            num += 1;
        }
        if self.second == pot {
            num += 1;
        }
        if self.third == pot {
            num += 1;
        }

        num
    }

    fn best(&self) -> MixologyPotion {
        max(max(self.first, self.second), self.third)
    }
}

#[allow(non_snake_case)]
#[derive(Default, Debug)]
struct MixologyPoints {
    M: usize,
    A: usize,
    L: usize,
}

impl std::ops::AddAssign for MixologyPoints {
    fn add_assign(&mut self, rhs: Self) {
        *self = MixologyPoints {
            M: self.M + rhs.M,
            A: self.A + rhs.A,
            L: self.L + rhs.L,
        }
    }
}

impl std::ops::Add for MixologyPoints {
    type Output = MixologyPoints;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::Mul<f32> for MixologyPoints {
    type Output = MixologyPoints;

    fn mul(self, rhs: f32) -> Self::Output {
        MixologyPoints {
            M: (self.M as f32 * rhs) as usize,
            A: (self.A as f32 * rhs) as usize,
            L: (self.L as f32 * rhs) as usize,
        }
    }
}

fn main() {
    // Generate the orders
    let orders = generate_orders(NUM_ORDERS_TO_SIM);

    // First strategy: complete every order
    let (input_pts, output_pts) = complete_every_order(&orders);
    println!("Complete All Orders:");
    println!("\tinput points:\n\t{:?}\n", input_pts);
    println!("\toutput points:\n\t{:?}\n", output_pts);
    let ratio_m = output_pts.M as f32 / input_pts.M as f32;
    let ratio_a = output_pts.A as f32 / input_pts.A as f32;
    let ratio_l = output_pts.L as f32 / input_pts.L as f32;
    println!(
        "\toutput:input:\n\tM: {:?}, A: {:?}, L: {:?}\n",
        ratio_m, ratio_a, ratio_l
    );

    // Second strategy: complete every order unless MAL exists, then only do MAL(s)
    let (input_pts, output_pts) = complete_every_order_unless_mal(&orders);
    println!("Complete All Orders Unless MAL, then only do MAL(s):");
    println!("\tinput points:\n\t{:?}\n", input_pts);
    println!("\toutput points:\n\t{:?}\n", output_pts);
    let ratio_m = output_pts.M as f32 / input_pts.M as f32;
    let ratio_a = output_pts.A as f32 / input_pts.A as f32;
    let ratio_l = output_pts.L as f32 / input_pts.L as f32;
    println!(
        "\toutput:input:\n\tM: {:?}, A: {:?}, L: {:?}\n",
        ratio_m, ratio_a, ratio_l
    );

    // Third strategy: complete single best order unless MAL exists, then only do MAL(s)
    let (input_pts, output_pts) = complete_best_order_unless_mal(&orders);
    println!("Complete Best Order Unless MAL, then do MAL(s):");
    println!("\tinput points:\n\t{:?}\n", input_pts);
    println!("\toutput points:\n\t{:?}\n", output_pts);
    let ratio_m = output_pts.M as f32 / input_pts.M as f32;
    let ratio_a = output_pts.A as f32 / input_pts.A as f32;
    let ratio_l = output_pts.L as f32 / input_pts.L as f32;
    println!(
        "\toutput:input:\n\tM: {:?}, A: {:?}, L: {:?}\n",
        ratio_m, ratio_a, ratio_l
    );

    // Fourth strategy: complete single best order unless MAL exists, then do all
    let (input_pts, output_pts) = complete_best_order_unless_mal_then_all(&orders);
    println!("Complete Best Order Unless MAL, then do all:");
    println!("\tinput points:\n\t{:?}\n", input_pts);
    println!("\toutput points:\n\t{:?}\n", output_pts);
    let ratio_m = output_pts.M as f32 / input_pts.M as f32;
    let ratio_a = output_pts.A as f32 / input_pts.A as f32;
    let ratio_l = output_pts.L as f32 / input_pts.L as f32;
    println!(
        "\toutput:input:\n\tM: {:?}, A: {:?}, L: {:?}\n",
        ratio_m, ratio_a, ratio_l
    );

    // Fifth strategy: don't to MMM, AAA, or LLL
    let (input_pts, output_pts) = complete_all_but_single_letter(&orders);
    println!("Don't do MMM, AAA, or LLL:");
    println!("\tinput points:\n\t{:?}\n", input_pts);
    println!("\toutput points:\n\t{:?}\n", output_pts);
    let ratio_m = output_pts.M as f32 / input_pts.M as f32;
    let ratio_a = output_pts.A as f32 / input_pts.A as f32;
    let ratio_l = output_pts.L as f32 / input_pts.L as f32;
    println!(
        "\toutput:input:\n\tM: {:?}, A: {:?}, L: {:?}\n",
        ratio_m, ratio_a, ratio_l
    );

    // Sixth strategy: complete best order unless MAL exists, then do all but single letter
    let (input_pts, output_pts) = complete_best_order_unless_mal_then_all_but_single(&orders);
    println!("Do Best Order Unless MAL, then do all but MMM, AAA, or LLL:");
    println!("\tinput points:\n\t{:?}\n", input_pts);
    println!("\toutput points:\n\t{:?}\n", output_pts);
    let ratio_m = output_pts.M as f32 / input_pts.M as f32;
    let ratio_a = output_pts.A as f32 / input_pts.A as f32;
    let ratio_l = output_pts.L as f32 / input_pts.L as f32;
    println!(
        "\toutput:input:\n\tM: {:?}, A: {:?}, L: {:?}\n",
        ratio_m, ratio_a, ratio_l
    );

    // Seventh strategy: do all but MMM, AAA, LLL, unles MAL exists, then do all
    let (input_pts, output_pts) = complete_all_but_single_letter_unless_mal_then_all(&orders);
    println!("Don't do MMM, AAA, or LLL, unless MAL exists then do all:");
    println!("\tinput points:\n\t{:?}\n", input_pts);
    println!("\toutput points:\n\t{:?}\n", output_pts);
    let ratio_m = output_pts.M as f32 / input_pts.M as f32;
    let ratio_a = output_pts.A as f32 / input_pts.A as f32;
    let ratio_l = output_pts.L as f32 / input_pts.L as f32;
    println!(
        "\toutput:input:\n\tM: {:?}, A: {:?}, L: {:?}\n",
        ratio_m, ratio_a, ratio_l
    );
}

fn generate_orders(num_orders: usize) -> Vec<MixologyOrder> {
    let mut orders = Vec::with_capacity(num_orders);

    let mut potion_sanity_check: HashMap<MixologyPotion, usize> = HashMap::new();

    let potions = all::<MixologyPotion>().collect::<Vec<_>>();

    let dist = WeightedAliasIndex::new(potions.iter().map(|pot| pot.weight()).collect()).unwrap();
    let mut rng = rand::rng();
    for _ in 0..num_orders {
        let first = potions[dist.sample(&mut rng)];
        let second = potions[dist.sample(&mut rng)];
        let third = potions[dist.sample(&mut rng)];
        orders.push(MixologyOrder {
            first,
            second,
            third,
        });

        potion_sanity_check
            .entry(first)
            .and_modify(|num| *num += 1)
            .or_insert(1);
        potion_sanity_check
            .entry(second)
            .and_modify(|num| *num += 1)
            .or_insert(1);
        potion_sanity_check
            .entry(third)
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    for pot in potions.iter() {
        println!(
            "{:?} was found {:?} times",
            pot,
            potion_sanity_check.get(pot).unwrap()
        );
    }

    orders
}

fn complete_every_order(orders: &[MixologyOrder]) -> (MixologyPoints, MixologyPoints) {
    let mut input_pts = MixologyPoints::default();
    let mut output_pts = MixologyPoints::default();

    let mul_factor = 1.4;

    for order in orders.iter() {
        input_pts += order.first.input_points();
        input_pts += order.second.input_points();
        input_pts += order.third.input_points();

        output_pts += order.first.output_points() * mul_factor;
        output_pts += order.second.output_points() * mul_factor;
        output_pts += order.third.output_points() * mul_factor;
    }

    (input_pts, output_pts)
}

fn complete_every_order_unless_mal(orders: &[MixologyOrder]) -> (MixologyPoints, MixologyPoints) {
    let mut input_pts = MixologyPoints::default();
    let mut output_pts = MixologyPoints::default();

    for order in orders.iter() {
        if order.contains(MixologyPotion::MAL) {
            let num_mal = order.num(MixologyPotion::MAL);
            let mul_factor = match num_mal {
                1 => 1.,
                2 => 1.2,
                3 => 1.4,
                _ => panic!("MORE THAN 3 POTS IN AN ORDER???"),
            };

            input_pts += MixologyPotion::MAL.input_points() * num_mal as f32;
            output_pts += (MixologyPotion::MAL.output_points() * mul_factor) * num_mal as f32;
        } else {
            let mul_factor = 1.4;

            input_pts += order.first.input_points();
            input_pts += order.second.input_points();
            input_pts += order.third.input_points();

            output_pts += order.first.output_points() * mul_factor;
            output_pts += order.second.output_points() * mul_factor;
            output_pts += order.third.output_points() * mul_factor;
        }
    }

    (input_pts, output_pts)
}

fn complete_best_order_unless_mal(orders: &[MixologyOrder]) -> (MixologyPoints, MixologyPoints) {
    let mut input_pts = MixologyPoints::default();
    let mut output_pts = MixologyPoints::default();

    for order in orders.iter() {
        if order.contains(MixologyPotion::MAL) {
            let num_mal = order.num(MixologyPotion::MAL);
            let mul_factor = match num_mal {
                1 => 1.,
                2 => 1.2,
                3 => 1.4,
                _ => panic!("MORE THAN 3 POTS IN AN ORDER???"),
            };

            input_pts += MixologyPotion::MAL.input_points() * num_mal as f32;
            output_pts += (MixologyPotion::MAL.output_points() * mul_factor) * num_mal as f32;
        } else {
            let best = order.best();
            input_pts += best.input_points();
            output_pts += best.output_points();
        }
    }

    (input_pts, output_pts)
}

fn complete_best_order_unless_mal_then_all(
    orders: &[MixologyOrder],
) -> (MixologyPoints, MixologyPoints) {
    let mut input_pts = MixologyPoints::default();
    let mut output_pts = MixologyPoints::default();

    for order in orders.iter() {
        if order.contains(MixologyPotion::MAL) {
            let mul_factor = 1.4;

            input_pts += order.first.input_points();
            input_pts += order.second.input_points();
            input_pts += order.third.input_points();

            output_pts += order.first.output_points() * mul_factor;
            output_pts += order.second.output_points() * mul_factor;
            output_pts += order.third.output_points() * mul_factor;
        } else {
            let best = order.best();
            input_pts += best.input_points();
            output_pts += best.output_points();
        }
    }

    (input_pts, output_pts)
}

fn complete_all_but_single_letter(orders: &[MixologyOrder]) -> (MixologyPoints, MixologyPoints) {
    let mut input_pts = MixologyPoints::default();
    let mut output_pts = MixologyPoints::default();

    for order in orders.iter() {
        let num_mmm = order.num(MixologyPotion::MMM);
        let num_aaa = order.num(MixologyPotion::AAA);
        let num_lll = order.num(MixologyPotion::LLL);
        let num_single = num_mmm + num_aaa + num_lll;

        if num_single == 3 || num_single == 2 {
            // Just do best and move on
            let best = order.best();
            input_pts += best.input_points();
            output_pts += best.output_points();
            continue;
        }

        let mul_factor = if num_single == 1 { 1.2 } else { 1.4 };

        if !(order.first == MixologyPotion::MMM
            || order.first == MixologyPotion::AAA
            || order.first == MixologyPotion::LLL)
        {
            input_pts += order.first.input_points();
            output_pts += order.first.output_points() * mul_factor;
        }
        if !(order.second == MixologyPotion::MMM
            || order.second == MixologyPotion::AAA
            || order.second == MixologyPotion::LLL)
        {
            input_pts += order.second.input_points();
            output_pts += order.second.output_points() * mul_factor;
        }
        if !(order.third == MixologyPotion::MMM
            || order.third == MixologyPotion::AAA
            || order.third == MixologyPotion::LLL)
        {
            input_pts += order.third.input_points();
            output_pts += order.third.output_points() * mul_factor;
        }
    }

    (input_pts, output_pts)
}

fn complete_best_order_unless_mal_then_all_but_single(
    orders: &[MixologyOrder],
) -> (MixologyPoints, MixologyPoints) {
    let mut input_pts = MixologyPoints::default();
    let mut output_pts = MixologyPoints::default();

    for order in orders.iter() {
        if order.contains(MixologyPotion::MAL) {
            let num_mmm = order.num(MixologyPotion::MMM);
            let num_aaa = order.num(MixologyPotion::AAA);
            let num_lll = order.num(MixologyPotion::LLL);
            let num_single = num_mmm + num_aaa + num_lll;

            if num_single == 2 {
                // Just do best and move on
                let best = order.best();
                input_pts += best.input_points();
                output_pts += best.output_points();
                continue;
            }

            let mul_factor = if num_single == 1 { 1.2 } else { 1.4 };

            if !(order.first == MixologyPotion::MMM
                || order.first == MixologyPotion::AAA
                || order.first == MixologyPotion::LLL)
            {
                input_pts += order.first.input_points();
                output_pts += order.first.output_points() * mul_factor;
            }
            if !(order.second == MixologyPotion::MMM
                || order.second == MixologyPotion::AAA
                || order.second == MixologyPotion::LLL)
            {
                input_pts += order.second.input_points();
                output_pts += order.second.output_points() * mul_factor;
            }
            if !(order.third == MixologyPotion::MMM
                || order.third == MixologyPotion::AAA
                || order.third == MixologyPotion::LLL)
            {
                input_pts += order.third.input_points();
                output_pts += order.third.output_points() * mul_factor;
            }
        } else {
            let best = order.best();
            input_pts += best.input_points();
            output_pts += best.output_points();
        }
    }

    (input_pts, output_pts)
}

fn complete_all_but_single_letter_unless_mal_then_all(
    orders: &[MixologyOrder],
) -> (MixologyPoints, MixologyPoints) {
    let mut input_pts = MixologyPoints::default();
    let mut output_pts = MixologyPoints::default();

    for order in orders.iter() {
        if order.contains(MixologyPotion::MAL) {
            let mul_factor = 1.4;

            input_pts += order.first.input_points();
            input_pts += order.second.input_points();
            input_pts += order.third.input_points();

            output_pts += order.first.output_points() * mul_factor;
            output_pts += order.second.output_points() * mul_factor;
            output_pts += order.third.output_points() * mul_factor;
        } else {
            let num_mmm = order.num(MixologyPotion::MMM);
            let num_aaa = order.num(MixologyPotion::AAA);
            let num_lll = order.num(MixologyPotion::LLL);
            let num_single = num_mmm + num_aaa + num_lll;

            if num_single == 3 || num_single == 2 {
                // Just do best and move on
                let best = order.best();
                input_pts += best.input_points();
                output_pts += best.output_points();
                continue;
            }

            let mul_factor = if num_single == 1 { 1.2 } else { 1.4 };

            if !(order.first == MixologyPotion::MMM
                || order.first == MixologyPotion::AAA
                || order.first == MixologyPotion::LLL)
            {
                input_pts += order.first.input_points();
                output_pts += order.first.output_points() * mul_factor;
            }
            if !(order.second == MixologyPotion::MMM
                || order.second == MixologyPotion::AAA
                || order.second == MixologyPotion::LLL)
            {
                input_pts += order.second.input_points();
                output_pts += order.second.output_points() * mul_factor;
            }
            if !(order.third == MixologyPotion::MMM
                || order.third == MixologyPotion::AAA
                || order.third == MixologyPotion::LLL)
            {
                input_pts += order.third.input_points();
                output_pts += order.third.output_points() * mul_factor;
            }
        }
    }

    (input_pts, output_pts)
}
