pub mod lib_follows;
pub mod lr1_generic;

#[cfg(test)]
mod test {
  use crate::lib_follows;
  use crate::lr1_generic::Symbol;
  use std::collections::VecDeque;
  #[test]
  fn follows_test() {
    let rule1 = ("S".to_string(), VecDeque::from(vec![Symbol::NonTerminal("E".to_string()), Symbol::Terminal("$".to_string())]));
    let rule2 = ("E".to_string(), VecDeque::from(vec![Symbol::NonTerminal("T".to_string()), Symbol::NonTerminal("G".to_string())]));
    let rule3 = ("G".to_string(), VecDeque::from(vec![Symbol::NonTerminal("".to_string())]));
    let rule4 = ("G".to_string(), VecDeque::from(vec![Symbol::Terminal("+".to_string()), Symbol::NonTerminal("E".to_string())]));
    let rule5 = ("G".to_string(), VecDeque::from(vec![Symbol::Terminal("-".to_string()), Symbol::NonTerminal("E".to_string())]));
    let rule6 = ("T".to_string(), VecDeque::from(vec![Symbol::NonTerminal("F".to_string()), Symbol::NonTerminal("H".to_string())]));
    let rule7 = ("H".to_string(), VecDeque::from(vec![Symbol::NonTerminal("".to_string())]));
    let rule8 = ("H".to_string(), VecDeque::from(vec![Symbol::Terminal("*".to_string()), Symbol::NonTerminal("T".to_string())]));
    let rule9 = ("F".to_string(), VecDeque::from(vec![Symbol::Terminal("(".to_string()), Symbol::NonTerminal("E".to_string()), Symbol::Terminal(")".to_string())]));
    let rule10 = ("F".to_string(), VecDeque::from(vec![Symbol::Terminal("id".to_string())]));
    let mut rules = VecDeque::from(vec![rule1, rule2, rule3, rule4, rule5, rule6, rule7, rule8, rule9, rule10]);
    let mut nulls = lib_follows::get_nulls(&mut rules);
    let mut firsts = lib_follows::get_firsts(&mut rules, &mut nulls);
    let follows = lib_follows::get_follows(&mut rules, &mut nulls, &mut firsts);
    let ans = VecDeque::from(vec![("S".to_string(), VecDeque::new()), ("E".to_string(), VecDeque::from(vec![")".to_string(), "$".to_string()])), ("G".to_string(), VecDeque::from(vec![")".to_string(), "$".to_string()])), ("T".to_string(), VecDeque::from(vec!["+".to_string(), ")".to_string(), "-".to_string(), "$".to_string()])), ("H".to_string(), VecDeque::from(vec!["+".to_string(), ")".to_string(), "-".to_string(), "$".to_string()])), ("F".to_string(), VecDeque::from(vec!["*".to_string(), "+".to_string(), ")".to_string(), "-".to_string(), "$".to_string()]))]);
    println!("{:?}", nulls);
    println!("{:?}", firsts);
    assert_eq!(ans, follows);
  }
}