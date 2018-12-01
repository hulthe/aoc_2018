#![feature(test)]
use std::collections::BTreeSet;
use std::error::Error;
use std::io::BufRead;

extern crate test;

fn parse_freqs<'a, R: BufRead + 'a>(reader: R) -> Box<(dyn Iterator<Item = i32> + 'a)> {
    let iter = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse::<i32>().ok());
    Box::new(iter)
}

pub fn sum_freqs<R: BufRead>(reader: R) -> Result<i32, Box<Error>> {
    let sum = parse_freqs(reader).sum();

    Ok(sum)
}

pub fn freqs_first_dup<R: BufRead>(reader: R) -> Result<i32, Box<Error>> {
    let pattern: Vec<i32> = parse_freqs(reader).collect();
    let mut history: BTreeSet<i32> = BTreeSet::new();

    let mut last = 0;
    loop {
        for num in pattern.iter() {
            if !history.insert(last) {
                return Ok(last);
            }
            last += num;
        }
    }
}

#[cfg(test)]
mod tests {
    use self::test::Bencher;
    use super::*;
    use std::io;

    #[test]
    fn sum_test() {
        let cursor = io::Cursor::new(b"+4\n-6\n+33");
        assert_eq!(sum_freqs(cursor).unwrap(), 31);
    }

    #[test]
    fn first_dup_test() {
        let mut cr = vec![
            ("1\n-1", 0),
            ("+3\n+3\n+4\n-2\n-4", 10),
            ("-6\n+3\n+8\n+5\n-6", 5),
            ("+7\n+7\n-2\n-7\n-4", 14),
            ("+9999\n+1\n-9999\n+1", 9999),
        ];
        let cases = cr.iter_mut().map(|(i, r)| (io::Cursor::new(i), r));

        for (i, r) in cases {
            assert_eq!(freqs_first_dup(i).unwrap(), r.clone());
        }
    }

    #[bench]
    fn bench_sum_range(b: &mut Bencher) {
        b.iter(sum_test)
    }

    #[bench]
    fn bench_find_dup(b: &mut Bencher) {
        b.iter(first_dup_test)
    }
}
