fn combo(operand: u32, ra: u32, rb: u32, rc: u32) -> u32 {
    let val = match operand {
        0 | 1 | 2 | 3 => operand,
        4 => ra,
        5 => rb,
        6 => rc,
        7 => panic!("nope"),
        _ => panic!("hmmm?"),
    };
    val
}

fn p1() -> String {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (r0, code) = input.split_once("\n\n").unwrap();

    let mut ra = r0
        .lines()
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut rb = r0
        .lines()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut rc = r0
        .lines()
        .nth(2)
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    // println!("{} {} {}", ra, rb, rc);

    let code: Vec<u32> = code
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // dbg!(&code);

    let mut ip: usize = 0;
    let mut out: Vec<u32> = vec![];

    while ip < code.len() {
        let operator = code[ip];
        let operand = code[ip + 1];
        match operator {
            0 => {
                ra = ra / (1 << combo(operand, ra, rb, rc));
                ip += 2;
            }
            1 => {
                rb = rb ^ operand;
                ip += 2;
            }
            2 => {
                rb = combo(operand, ra, rb, rc) % 8;
                ip += 2;
            }
            3 => {
                if ra != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                rb = rb ^ rc;
                ip += 2;
            }
            5 => {
                out.push(combo(operand, ra, rb, rc) % 8);
                ip += 2;
            }
            6 => {
                rb = ra / (1 << combo(operand, ra, rb, rc));
                ip += 2;
            }
            7 => {
                rc = ra / (1 << combo(operand, ra, rb, rc));
                ip += 2;
            }
            _ => panic!(),
        }
    }

    // dbg!(&out);
    // dbg!(&ra, &rb, &rc);

    out.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn p2() -> i32 {
    let _input = std::fs::read_to_string("input.txt").unwrap();
    0
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
