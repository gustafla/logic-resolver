use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::fmt;
use log::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Literal {
    pub m: u32,
    pub negated: bool,
}

impl fmt::Display for Literal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.negated {
            write!(fmt, "¬")?;
        }
        write!(fmt, "p{}", self.m)
    }
}

impl Ord for Literal {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.m.cmp(&other.m) {
            Ordering::Equal => self.negated.cmp(&other.negated),
            other => other,
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Literal {
    fn normalize(&self) -> Self {
        Self {
            m: self.m,
            negated: false,
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub struct Clause(BTreeSet<Literal>);

impl fmt::Display for Clause {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            write!(fmt, "∅")
        } else {
            write!(fmt, "{{")?;
            for lit in self.0.iter().take(self.0.len() - 1) {
                write!(fmt, "{}, ", lit)?;
            }
            if let Some(last) = self.0.iter().last() {
                write!(fmt, "{}", last)?;
            }
            write!(fmt, "}}")
        }
    }
}

impl Clause {
    pub fn new(lits: &[Literal]) -> Self {
        Self(lits.iter().cloned().collect())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// target has to be from self (has to have the same sign)
    fn rule_out(&self, with: &Clause, target: &Literal) -> Self {
        let mut set = self.0.clone();
        assert!(set.remove(target));
        for &literal in with.0.iter() {
            if literal.m != target.m || literal.negated == target.negated {
                set.insert(literal);
            }
        }
        Self(set)
    }
}

#[derive(Clone, Debug)]
pub struct Statement(HashSet<Clause>);

impl Statement {
    pub fn new(clauses: &[Clause]) -> Self {
        Self(clauses.iter().cloned().collect())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn insert(&mut self, clause: Clause) -> bool {
        self.0.insert(clause)
    }

    pub fn resolve(&self) -> bool {
        let mut knowledge_base = self.clone();
        let kb_size = knowledge_base.len();
        for clause1 in &self.0 {
            for literal1 in &clause1.0 {
                for clause2 in &self.0 {
                    for literal2 in &clause2.0 {
                        if literal1.m == literal2.m && literal1.negated != literal2.negated {
                            let new_clause = clause1.rule_out(&clause2, literal1);
                            info!(
                                "Ruling out {} from {} and {} producing {}",
                                literal1.normalize(),
                                clause1,
                                clause2,
                                new_clause
                            );
                            if new_clause.is_empty() {
                                return true;
                            }
                            if knowledge_base.insert(new_clause) {
                                info!("New sequent found");
                            }
                        }
                    }
                }
            }
        }
        if knowledge_base.len() == kb_size {
            false
        } else {
            knowledge_base.resolve()
        }
    }
}
