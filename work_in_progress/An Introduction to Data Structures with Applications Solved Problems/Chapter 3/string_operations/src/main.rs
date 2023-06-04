fn main() {
    let mut p = ['\0'; 20];
    let mut q = ['\0'; 20];
    p[0] = 'H'; p[1] = 'e'; p[2] = 'l'; p[3] = 'l'; p[4] = 'o'; p[5] = ' ';
    p[6] = 'w'; p[7] = 'o'; p[8] = 'r'; p[9] = 'l'; p[10] = 'd'; p[11] = '!';
    q[0] = 'l'; q[1] = 'l'; q[2] = 'o';
    let s = substring(&p, 11, 1);
    println!("Substring array length: {}", length(&s));
    println!("Substring array: {:?}", s);
    println!("Initial array: {:?}", p);
    println!("Array length: {}", length(&p));
    println!("Pattern 'oob' index in the string array is: {}", index_bf(&p, &q));

}

fn length(s: &[char; 20]) -> usize {
    let mut i: usize = 0;
    while 20 >= i {
        if '\0' == s[i] {
            return i;
        }

        i = i + 1;
    }

    i

}

fn substring(s: &[char; 20], k: usize, l: usize) -> [char; 20] {
    let mut result = ['\0'; 20];
    if k + l <= length(s) {
        let mut i = k;

        while k + l > i {
            result[i - k] = s[i];

            i = i + 1;

        }

    }

    result

}

fn index_bf(s: &[char; 20], p: &[char; 20]) -> usize {
    let ls = length(&s);
    let lp = length(&p);

    if lp <= ls {
        let mut i: usize = 0;

        while ls - lp + 1 >= i {
            let mut j: usize = 0;

            while lp >= j {
                if s[i + j] != p[j] {
                    break;

                }

                j = j + 1;

            }

            if lp == j {
                return i;

            }

            i = i + 1;

        }

    }

    return 21 as usize;

}
