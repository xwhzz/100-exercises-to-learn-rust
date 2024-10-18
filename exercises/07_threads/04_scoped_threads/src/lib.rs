// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint = v.len() /2;
    let mut r1 = 0;
    let mut r2 = 0;
    let r1ref = &mut r1;
    let r2ref = &mut r2;
    std::thread::scope(|scope| {
        scope.spawn(||{
            let first = &v[..midpoint];
            *r1ref = first.iter().sum();
        });
        scope.spawn(|| {
            let second = &v[midpoint..];
            *r2ref = second.iter().sum();
        });
    });
    *r1ref + *r2ref
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
