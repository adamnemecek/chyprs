mod ruletac;
mod simptac;

// use crate::graph::Graph;
// use crate::rewrite::dpo;
// use crate::rule::{Rule, RuleError};
// use crate::matcher::{Match, match_rule, find_iso};
// use crate::state;
use crate::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

// use regex::Regex;

pub struct Tactic<'a, T> {
    // state: &'a mut state::State,
    // local_state: &'a mut state::RewriteState,
    context: HashMap<String, Rule<T>>,
    goal_lhs: &'a Graph<T>,
    goal_rhs: &'a Graph<T>,
    errors: HashSet<String>,
    args: Vec<String>,
}

// lazy_static! {
//     static ref RULE_NAME_RE: Regex = Regex::new(r"(-)?\s*([a-zA-Z_][\.a-zA-Z0-9_]*)").unwrap();
// }

// impl<'a> Tactic<'a> {
//     pub fn new(local_state: &'a mut state::RewriteState, args: Vec<String>) -> Tactic<'a> {
//         Self {
//             local_state,
//             state: &mut local_state.state,
//             context: HashMap::new(),
//             goal_lhs: None,
//             goal_rhs: None,
//             errors: HashSet::new(),
//             args,
//         }
//     }

//     pub fn repeat<F>(
//         rw: F,
//         rules: Vec<String>,
//         max_iter: usize,
//     ) where
//         F: Fn(&str) -> bool,
//     {
//         let mut got_match = true;
//         let mut i = 0;
//         while got_match && i < max_iter {
//             got_match = false;
//             for r in &rules {
//                 while rw(r) && i < max_iter {
//                     got_match = true;
//                     i += 1;
//                 }
//             }
//         }
//     }

//     pub fn error(&mut self, message: &str) {
//         if !self.errors.contains(message) {
//             self.state
//                 .errors
//                 .push((self.state.file_name.clone(), self.local_state.line_number, message.to_string()));
//             self.errors.insert(message.to_string());
//         }
//     }

//     pub fn has_goal(&self) -> bool {
//         self.goal_lhs.is_some() && self.goal_rhs.is_some()
//     }

//     pub fn global_rules(&self) -> Vec<String> {
//         self.state.rule_sequence
//             .iter()
//             .filter(|(_, &j)| j <= self.local_state.sequence)
//             .map(|(name, _)| name.clone())
//             .collect()
//     }

//     pub fn lookup_rule(&self, rule_expr: &str, local: Option<bool>) -> (Option<Rule>, bool) {
//         let m = RULE_NAME_RE.captures(rule_expr);
//         if m.is_none() {
//             self.error(&format!("Bad rule expression: {}", rule_expr));
//             return (None, false);
//         }
//         let captures = m.unwrap();
//         let converse = captures.get(1).map(|m| m.as_str() == "-").unwrap_or(false);
//         let rule_name = captures.get(2).unwrap().as_str();

//         let loc = local.unwrap_or(true);
//         let glo = local.unwrap_or(false);

//         let mut rule: Option<Rule> = None;
//         if loc && self.context.contains_key(rule_name) {
//             rule = self.context.get(rule_name).cloned();
//         }

//         if glo && rule.is_none() && self.state.rule_sequence.contains_key(rule_name) {
//             if let Some(j) = self.state.rule_sequence.get(rule_name) {
//                 if *j.<= self.local_state.sequence
//                 {
//                     rule = Some(self.state.rule_map[&j].clone());
//                 }
//             }
//         }

//         if rule.is_none() {
//             self.error(&format!("Rule '{}' not found", rule_name));
//         }

//         (rule, converse)
//     }

//     pub fn apply_rule(&mut self, rule_expr: &str, local: Option<bool>) -> Result<(), RuleError> {
//         let (rule, converse) = self.lookup_rule(rule_expr, local);
//         if rule.is_none() {
//             return Err(RuleError::NotFound);
//         }

//         let rule = rule.unwrap();
//         let (lhs, rhs) = if converse {
//             (rule.rhs.clone(), rule.lhs.clone())
//         } else {
//             (rule.lhs.clone(), rule.rhs.clone())
//         };

//         let match_result = match_rule(&lhs, &self.goal_lhs.clone().unwrap());

//         if let Some(matching) = find_iso(&match_result) {
//             let dpo_result = dpo(&lhs, &rhs, &matching);
//             if dpo_result.is_ok() {
//                 self.goal_lhs = Some(dpo_result.unwrap().lhs);
//                 self.goal_rhs = Some(dpo_result.unwrap().rhs);
//                 Ok(())
//             } else {
//                 self.error(&format!("Rule application failed: {:?}", dpo_result.unwrap_err()));
//                 Err(RuleError::ApplicationFailed)
//             }
//         } else {
//             self.error("No match found");
//             Err(RuleError::NoMatchFound)
//         }
//     }
// }
