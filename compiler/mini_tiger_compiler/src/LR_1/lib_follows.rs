use std::collections::VecDeque;
//use crate::src::LR_1::lr1_generic::Symbol;
use super::lr1_generic::Symbol;

fn merge_and_unity<T: Eq + Ord + Clone>(l1: &mut VecDeque<T>, l2: &mut VecDeque<T>) -> VecDeque<T> {
    if l1.is_empty() {
        return l2.clone();
    } else if l2.is_empty() {
        return l1.clone();
    } else {
        let x = l1[0].clone();
        let y = l2[0].clone();
        let mut l3 = l1.clone();
        let mut l4 = l2.clone();
        if x == y {
            l3.pop_front().unwrap();
            l4.pop_front().unwrap();
            let mut l = merge_and_unity(&mut l3, &mut l4);
            l.push_front(x);
            return l;
        } else if x < y {
            l3.pop_front().unwrap();
            let mut l = merge_and_unity(&mut l3, &mut l4);
            l.push_front(x);
            return l;
        } else {
            l4.pop_front().unwrap();
            let mut l = merge_and_unity(&mut l3, &mut l4);
            l.push_front(y);
            return l;
        }
    }
}

fn merge_lists<T: Eq + Ord + Clone>(ll: &mut VecDeque<VecDeque<T>>) -> VecDeque<T> {
    ll.iter().fold(VecDeque::new(), |mut acc, l| {
        merge_and_unity(&mut acc, &mut l.clone())
    })
}

fn list_tl_n<T: Clone>(l: &mut VecDeque<T>, n: usize) -> VecDeque<T> {
    if n == 0 {
        l.clone()
    } else {
        let mut l1 = l.clone();
        l1.pop_front().unwrap();
        list_tl_n(&mut l1, n - 1)
    }
}

fn delete_duplication<T: Eq + Ord + Clone>(l: &mut VecDeque<T>) -> VecDeque<T> {
    if l.is_empty() {
        l.clone()
    } else if l.len() == 1 {
        l.clone()
    } else {
        let x = l[0].clone();
        let y = l[1].clone();
        let mut l1 = l.clone();
        if x == y {
            l1.pop_front().unwrap();
            delete_duplication(&mut l1)
        } else {
            l1.pop_front().unwrap();
            l1 = delete_duplication(&mut l1);
            l1.push_front(x);
            l1
        }
    }
}

fn lfp_2<T, F, S>(mut f: F, n: &mut VecDeque<T>, x: &mut VecDeque<S>) -> VecDeque<S>
where
    F: FnMut(&mut VecDeque<T>, &mut VecDeque<S>) -> VecDeque<S>,
    S: Eq + Ord,
{
    let mut y = f(n, x);
    if y == *x {
        y
    } else {
        lfp_2(f, n, &mut y)
    }
}

fn lfp_3<T, F, S, U>(
    mut f: F,
    n: &mut VecDeque<T>,
    m: &mut VecDeque<U>,
    x: &mut VecDeque<S>,
) -> VecDeque<S>
where
    F: FnMut(&mut VecDeque<T>, &mut VecDeque<U>, &mut VecDeque<S>) -> VecDeque<S>,
    S: Eq + Ord,
{
    let mut y = f(n, m, x);
    if y == *x {
        y
    } else {
        lfp_3(f, n, m, &mut y)
    }
}

fn lfp_4<T, F, S, U, V>(
    mut f: F,
    n: &mut VecDeque<T>,
    m: &mut VecDeque<U>,
    o: &mut VecDeque<V>,
    x: &mut VecDeque<S>,
) -> VecDeque<S>
where
    F: FnMut(&mut VecDeque<T>, &mut VecDeque<U>, &mut VecDeque<V>, &mut VecDeque<S>) -> VecDeque<S>,
    S: Eq + Ord,
{
    let mut y = f(n, m, o, x);
    if y == *x {
        y
    } else {
        lfp_4(f, n, m, o, &mut y)
    }
}

fn step_null(
    rules: &mut VecDeque<(String, VecDeque<Symbol>)>,
    nulls: &mut VecDeque<String>,
) -> VecDeque<String> {
    let rules2: VecDeque<(String, VecDeque<Symbol>)> = rules
        .iter()
        .filter(|&r1| {
            r1.1.iter().all(|r2| match r2 {
                Symbol::Terminal(_) => false,
                Symbol::NonTerminal(s) => nulls.iter().any(|n| *n == *s),
            })
        }).map(|r| r.clone())
        .collect();

    let mut nulls2 = delete_duplication(&mut rules2.iter().map(|r| r.0.clone()).collect());

    merge_and_unity(&mut nulls2, nulls)
}

pub fn get_nulls(rules: &mut VecDeque<(String, VecDeque<Symbol>)>) -> VecDeque<String> {
    lfp_2(step_null, rules, &mut VecDeque::from(vec!["".to_string()]))
}

fn null_alpha(nulls: &mut VecDeque<String>, alpha: &mut VecDeque<Symbol>) -> bool {
    if alpha.is_empty() {
        true
    } else {
        match alpha[0].clone() {
            Symbol::Terminal(_) => false,
            Symbol::NonTerminal(s) => {
                let mut alpha2 = alpha.clone();
                alpha2.pop_front().unwrap();
                nulls.iter().any(|n| *n == *s) && null_alpha(nulls, &mut alpha2)
            }
        }
    }
}

fn step_first(
    rules: &mut VecDeque<(String, VecDeque<Symbol>)>,
    nulls: &mut VecDeque<String>,
    firsts: &mut VecDeque<(String, VecDeque<String>)>,
) -> VecDeque<(String, VecDeque<String>)> {
    firsts
        .iter()
        .map(|r| {
            (
                r.0.clone(),
                merge_and_unity(&mut r.1.clone(), &mut step_first_nt(rules, nulls, &mut r.0.clone(), &mut firsts.clone())),
            )
        })
        .collect()
}

fn step_first_nt(
    rules: &mut VecDeque<(String, VecDeque<Symbol>)>,
    nulls: &mut VecDeque<String>,
    nt: &mut String,
    firsts: &mut VecDeque<(String, VecDeque<String>)>,
) -> VecDeque<String> {
    let rules2: VecDeque<(String, VecDeque<Symbol>)> =
        rules.iter().filter(|r| *nt == r.0).map(|r| r.clone()).collect();
    let mut ttlists = rules2
        .iter()
        .map(|r| step_first_alpha(nulls, &mut r.1.clone(), firsts))
        .collect();
    merge_lists(&mut ttlists)
}

fn step_first_alpha(
    nulls: &mut VecDeque<String>,
    alpha: &mut VecDeque<Symbol>,
    firsts: &mut VecDeque<(String, VecDeque<String>)>,
) -> VecDeque<String> {
    if alpha.is_empty() {
        VecDeque::new()
    } else {
        match alpha[0].clone() {
            Symbol::Terminal(t) => {
                let mut ans = VecDeque::new();
                ans.push_back(t);
                ans
            }
            Symbol::NonTerminal(s) => {
                let mut terminals = match firsts.clone().iter().find(|r| r.0 == s).clone()
                {
                    Some(r) => r.1.clone(),
                    None => VecDeque::new(),
                };
                let mut alpha2 = alpha.clone();
                alpha2.pop_front().unwrap();
                match nulls.iter().find(|n| **n == s) {
                    Some(_) => merge_and_unity(
                        &mut terminals,
                        &mut step_first_alpha(nulls, &mut alpha2, firsts),
                    ),
                    None => terminals,
                }
            }
        }
    }
}

pub fn get_firsts(
    rules: &mut VecDeque<(String, VecDeque<Symbol>)>,
    nulls: &mut VecDeque<String>,
) -> VecDeque<(String, VecDeque<String>)> {
    let nts = delete_duplication(&mut rules.iter().map(|r| r.0.clone()).collect());
    let mut bot = nts
        .iter()
        .map(|nt| (nt.clone(), VecDeque::new()))
        .collect();
    lfp_3(step_first, rules, nulls, &mut bot)
}

fn first_alpha(
    nulls: &mut VecDeque<String>,
    firsts: &mut VecDeque<(String, VecDeque<String>)>,
    alpha: &mut VecDeque<Symbol>,
) -> VecDeque<String> {
    if alpha.is_empty() {
        VecDeque::new()
    } else {
        match alpha[0].clone() {
            Symbol::Terminal(t) => {
                let mut ans = VecDeque::new();
                ans.push_back(t);
                ans
            }
            Symbol::NonTerminal(s) => {
                let mut terminals = match firsts.clone().iter().find(|r| r.0 == s).clone()
                {
                    Some(r) => r.1.clone(),
                    None => VecDeque::new(),
                };
                let mut alpha2 = alpha.clone();
                alpha2.pop_front().unwrap();
                match nulls.iter().find(|n| **n == s) {
                    Some(_) => {
                        merge_and_unity(&mut terminals, &mut first_alpha(nulls, firsts, &mut alpha2))
                    }
                    None => terminals,
                }
            }
        }
    }
}

fn step_follow(
    rules: &mut VecDeque<(String, VecDeque<Symbol>)>,
    nulls: &mut VecDeque<String>,
    firsts: &mut VecDeque<(String, VecDeque<String>)>,
    follows: &mut VecDeque<(String, VecDeque<String>)>,
) -> VecDeque<(String, VecDeque<String>)> {
    follows
        .iter()
        .map(|r| {
            (
                r.0.clone(),
                merge_and_unity(&mut r.1.clone(), &mut step_follow_nt(rules, nulls, firsts, &mut follows.clone(), &mut r.0.clone())),
            )
        })
        .collect()
}

fn step_follow_nt(
    rules: &mut VecDeque<(String, VecDeque<Symbol>)>,
    nulls: &mut VecDeque<String>,
    firsts: &mut VecDeque<(String, VecDeque<String>)>,
    follows: &mut VecDeque<(String, VecDeque<String>)>,
    nt: &mut String,
) -> VecDeque<String> {
    merge_lists(
        &mut rules
            .iter()
            .map(|r| {
                step_follow_alpha(
                    nulls,
                    firsts,
                    follows,
                    nt,
                    &mut r.1.clone(),
                    &mut follows.clone().iter().find(|s| s.0 == r.0).unwrap().1.clone(),
                )
            })
            .collect(),
    )
}

fn step_follow_alpha(
    nulls: &mut VecDeque<String>,
    firsts: &mut VecDeque<(String, VecDeque<String>)>,
    follows: &mut VecDeque<(String, VecDeque<String>)>,
    nt: &mut String,
    alpha: &mut VecDeque<Symbol>,
    follow: &mut VecDeque<String>,
) -> VecDeque<String> {
    if alpha.is_empty() {
        VecDeque::new()
    } else {
        let mut alpha2 = alpha.clone();
        alpha2.pop_front();
        match alpha[0].clone() {
            Symbol::Terminal(_) => {
                step_follow_alpha(nulls, firsts, follows, nt, &mut alpha2, follow)
            }
            Symbol::NonTerminal(s) => {
                let follow0 = step_follow_alpha(nulls, firsts, follows, nt, &mut alpha2, follow);
                if s == *nt {
                    let follow1 = first_alpha(nulls, firsts, &mut alpha2);
                    let follow2 = if null_alpha(nulls, &mut alpha2) {
                        follow.clone()
                    } else {
                        VecDeque::new()
                    };
                    let mut follow4 = VecDeque::new();
                    follow4.push_back(follow0);
                    follow4.push_back(follow1);
                    follow4.push_back(follow2);
                    merge_lists(&mut follow4)
                } else {
                    follow0
                }
            }
        }
    }
}
pub fn get_follows(
    rules: &mut VecDeque<(String, VecDeque<Symbol>)>,
    nulls: &mut VecDeque<String>,
    firsts: &mut VecDeque<(String, VecDeque<String>)>,
) -> VecDeque<(String, VecDeque<String>)> {
    let nts = delete_duplication(&mut rules.iter().map(|r| r.0.clone()).collect());
    let mut bot = nts
        .iter()
        .map(|nt| (nt.clone(), VecDeque::new()))
        .collect();
    lfp_4(step_follow, rules, nulls, firsts, &mut bot)
}

fn terminals_in_alpha(alpha: &mut VecDeque<Symbol>) -> VecDeque<String> {
    if alpha.is_empty() {
        VecDeque::new()
    } else {
        match alpha[0].clone() {
            Symbol::Terminal(t) => {
                let mut ans = VecDeque::new();
                ans.push_back(t);
                let mut alpha2 = alpha.clone();
                alpha2.pop_front();
                merge_and_unity(
                    &mut ans,
                    &mut terminals_in_alpha(&mut alpha2),
                )
            }
            Symbol::NonTerminal(_) => {
                let mut alpha2 = alpha.clone();
                alpha2.pop_front();
                terminals_in_alpha(&mut alpha2)
            }
        }
    }
}

fn terminal_in_rules(rules: &mut VecDeque<(String, VecDeque<Symbol>)>) -> VecDeque<String> {
    merge_lists(&mut rules.iter().map(|r| terminals_in_alpha(&mut r.1.clone())).collect())
}
