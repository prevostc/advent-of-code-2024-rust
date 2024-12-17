advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct Machine {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    ip: usize, // idx into tape
    tape: Vec<u64>,
    output: Vec<u64>,
}

impl Machine {
    fn parse(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        Self {
            reg_a: lines[0].split_whitespace().nth(2).unwrap().parse().unwrap(),
            reg_b: lines[1].split_whitespace().nth(2).unwrap().parse().unwrap(),
            reg_c: lines[2].split_whitespace().nth(2).unwrap().parse().unwrap(),
            ip: 0,
            tape: lines[4]
                .split_whitespace()
                .nth(1)
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
            output: vec![],
        }
    }

    #[inline]
    fn combo(&self, op: u64) -> u64 {
        match op {
            0..=3 => op,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn literal(&self, op: u64) -> u64 {
        return op;
    }

    #[inline]
    fn _div(&self, op: u64) -> u64 {
        let l = self.reg_a;
        let r = 2_u64.pow(self.combo(op) as u32);
        l.div_euclid(r)
    }

    #[inline]
    fn adv(&mut self, op: u64) {
        self.reg_a = self._div(op);
    }

    #[inline]
    fn bdv(&mut self, op: u64) {
        self.reg_b = self._div(op);
    }

    #[inline]
    fn cdv(&mut self, op: u64) {
        self.reg_c = self._div(op);
    }

    #[inline]
    fn bxl(&mut self, op: u64) {
        self.reg_b = self.reg_b ^ self.literal(op);
    }

    #[inline]
    fn bst(&mut self, op: u64) {
        self.reg_b = self.combo(op) % 8;
    }

    #[inline]
    fn jnz(&mut self, op: u64) {
        if self.reg_a != 0 {
            self.ip = self.literal(op) as usize;
        }
    }

    #[inline]
    fn bxc(&mut self, _op: u64) {
        self.reg_b = self.reg_b ^ self.reg_c;
    }

    #[inline]
    fn out(&mut self, op: u64) {
        let v = self.combo(op) % 8;
        self.output.push(v);
    }

    pub fn step(&mut self) -> bool {
        if self.ip + 1 >= self.tape.len() {
            return false;
        }

        let (inst, op) = (self.tape[self.ip], self.tape[self.ip + 1]);
        self.ip += 2;

        match inst {
            0 => self.adv(op),
            1 => self.bxl(op),
            2 => self.bst(op),
            3 => self.jnz(op),
            4 => self.bxc(op),
            5 => self.out(op),
            6 => self.bdv(op),
            7 => self.cdv(op),
            _ => unreachable!(),
        }

        return true;
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut machine = Machine::parse(input);

    while machine.step() {}

    Some(
        machine
            .output
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let machine = Machine::parse(input);
    let len = machine.tape.len();

    let mut reg_a = 0;
    for take_len in 1..=len {
        let mut new_a = reg_a << 3;
        loop {
            let mut m = machine.clone();
            m.reg_a = new_a;

            while m.step() {}

            if m.output.eq(&machine.tape[len - take_len..]) {
                reg_a = new_a;
                break;
            }

            new_a += 1;
        }
    }

    Some(reg_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ex_1() {
    //     let mut machine = Machine {
    //         reg_a: 0,
    //         reg_b: 0,
    //         reg_c: 9,
    //         ip: 0,
    //         tape: vec![2, 6],
    //         output: vec![],
    //     };
    //     machine.step();
    //     assert_eq!(machine.reg_b, 1);
    // }

    // #[test]
    // fn test_ex_2() {
    //     let mut machine = Machine {
    //         reg_a: 10,
    //         reg_b: 0,
    //         reg_c: 0,
    //         ip: 0,
    //         tape: vec![5, 0, 5, 1, 5, 4],
    //         output: vec![],
    //     };
    //     while machine.step() {}
    //     assert_eq!(machine.output, vec![0, 1, 2]);
    // }

    // #[test]
    // fn test_ex_3() {
    //     let mut machine = Machine {
    //         reg_a: 2024,
    //         reg_b: 0,
    //         reg_c: 0,
    //         ip: 0,
    //         tape: vec![0, 1, 5, 4, 3, 0],
    //         output: vec![],
    //     };
    //     while machine.step() {}
    //     assert_eq!(machine.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    //     assert_eq!(machine.reg_a, 0);
    // }

    // #[test]
    // fn test_ex_4() {
    //     let mut machine = Machine {
    //         reg_a: 0,
    //         reg_b: 29,
    //         reg_c: 0,
    //         ip: 0,
    //         tape: vec![1, 7],
    //         output: vec![],
    //     };
    //     machine.step();
    //     assert_eq!(machine.reg_b, 26);
    // }

    // #[test]
    // fn test_ex_5() {
    //     let mut machine = Machine {
    //         reg_a: 0,
    //         reg_b: 2024,
    //         reg_c: 43690,
    //         ip: 0,
    //         tape: vec![4, 0],
    //         output: vec![],
    //     };
    //     machine.step();
    //     assert_eq!(machine.reg_b, 44354);
    // }

    // #[test]
    // fn test_ex_6() {
    //     let mut machine = Machine {
    //         reg_a: 117440,
    //         reg_b: 0,
    //         reg_c: 0,
    //         ip: 0,
    //         tape: vec![0, 3, 5, 4, 3, 0],
    //         output: vec![],
    //     };
    //     while machine.step() {}
    //     assert_eq!(machine.output, vec![0, 3, 5, 4, 3, 0]);
    // }

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
    //     assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
