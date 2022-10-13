use super::lib_follows;
use thiserror::Error;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
}

impl Symbol {
    pub fn is_terminal(&self) -> bool {
        match self {
            Symbol::Terminal(_) => true,
            Symbol::NonTerminal(_) => false,
        }
    }

    pub fn content(&self) -> String {
        match self {
            Symbol::Terminal(s) => s.clone(),
            Symbol::NonTerminal(s) => s.clone(),
        }
    }
}

type Rule = String, VecDeque<Symbol>;

struct Grammer {
    rule: VecDeque<Rule>,
    s: String,
}

impl Grammer {
    pub fn new(rule: rule, s: String) -> Self {
        Self { rule, s }
    }
}

enum ParseTree {
    Terminal(String),
    NonTerminal(String, Vec<ParseTree>),
}

type Pos = usize;
type Item = (Rule, Pos, VecDeque<String>);
type State = VecDeque<Item>;

type Stack = (Symbol, ParseTree, State);

enum Action {
    Shift(State),
    Reduce(Rule),
    Accept,
}

const EOF: String = "$".to_string();

#[derive(Debug, Error)]
pub enum Lr1Error {
    
}


struct Lr_1 {
    rules: VecDeque<(String, VecDeque<Symbol>)>,
    nulls: VecDeque<String>,
    firsts: VecDeque<(String, VecDeque<String>)>,
    follows: VecDeque<(String, VecDeque<String>)>,
    states: VecDeque<State>,
    input_buffer: VecDeque<String>,
    current_pos: usize,
}

impl Lr_1 {
    pub fn new(rules: VecDeque<(String, VecDeque<Symbol>)>, nulls: VecDeque<String>, firsts: VecDeque<(String, VecDeque<String>)>, follows: VecDeque<(String, VecDeque<String>)>) -> Self {
        Self { rules, nulls, firsts, follows, states: VecDeque::new(), input_buffer: Vec::new(), current_pos: 0 }
    }
    
    fn read_token(&mut self) -> String {
        let token = self.input_buffer[self.current_pos].clone();
        self.current_pos += 1;
        token
    }

    fn unread_token(&mut self) {
        self.current_pos -= 1;
    }

    fn closure(&mut self, &mut st, &mut rules) -> VecDeque<Item> {
    let get_epsilontr = |item| {
        let mut ((nt, alpha), pos, cs) = item;
        if pos >= alpha.len() {
            VecDeque::new()
        } else {
            let mut rest = lib_follows::list_tl_n(&mut alpha, pos);
            let mut x = rest.pop_front().unwrap();
            let mut first = lib_follows::first_alpha(&mut !self.nulls, &mut !self.firsts, &mut rest);
            let mut first2 = 
        }
    }
}

}

fn subset<T: Eq>(cs1: &mut VecDeque<T>, cs2: &mut VecDeque<T>) -> bool {
    cs1.iter().all(|c| cs2.iter().any(|c2| *c == *c2))
}

fn item_subsumed(&mut item1: Item, &mut item2: Item) -> bool {
    let (rule1, pos1, cs1) = *item1;
    let (rule2, pos2, cs2) = *item2;
    (rule1 == rule2) && (pos1 == pos2) && subset(cs1, cs2)
}

fn merge_items(items1; &mut VecDeque<Item>, items2: &mut VecDeque<Item>) -> VecDeque<Item> {
    if items1.len() == 0 {
        return items2.clone();
    } else if items2.len() == 0 {
        return items1.clone();
    } else {
        let mut items3 = items1.clone();
        let mut items4 = items2.clone();
        (rule1, pos1, cs1) = items3.pop_front().unwrap();
        (rule2, pos2, cs2) = items4.pop_front().unwrap();
        if (rule1, pos1) < (rule2, pos2) {
            let mut ans = VecDeque::new();
            ans.push_back((rule1, pos1, cs1));
            ans.append(&mut merge_items(&mut items3, items2));
        } else if (rule1, pos1) > (rule2, pos2) {
            let mut ans = VecDeque::new();
            ans.push_back((rule2, pos2, cs2));
            ans.append(&mut merge_items(items1, &mut items4));
        } else {
            let mut ans = VecDeque::new();
            ans.push_back((rule1, pos1, lib_follows::merge_and_unity(&mut cs1, &mut cs2)));
            ans.append(&mut merge_items(&mut items3, &mut items4));
        }
    }
}

