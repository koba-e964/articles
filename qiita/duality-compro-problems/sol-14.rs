use std::cmp::*;
use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn solve(a: &[i64]) -> Vec<i64> {
    let mut que = BinaryHeap::new();
    let mut global = 0;
    for a in a {
        global -= a;
        que.push(Reverse(a));
        que.push(Reverse(a));
        let Reverse(x) = que.pop().unwrap();
        global += x;
    }
    let v = que.into_sorted_vec();
    let mut ans = vec![global];
    for Reverse(v) in v.into_iter().rev() {
        let new = ans[ans.len() - 1] + v;
        ans.push(new);
    }
    ans
}

fn main() {
    let mut a: Vec<i64> = getline().trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    a.reverse();
    let ans = solve(&a);
    eprintln!("{:?}", ans);
    println!("{}", -ans[0]);
}
