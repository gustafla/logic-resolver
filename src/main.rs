use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Literal {
    m: u32,
    negated: bool,
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
struct Clause(Vec<Literal>);

impl fmt::Display for Clause {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            write!(fmt, "∅")
        } else {
            write!(fmt, "{{")?;
            for lit in self.0.iter().take(self.0.len() - 1) {
                write!(fmt, "{}, ", lit)?;
            }
            if let Some(last) = self.0.last() {
                write!(fmt, "{}", last)?;
            }
            write!(fmt, "}}")
        }
    }
}

impl Clause {
    fn new(lits: &[Literal]) -> Self {
        let mut vec = lits.to_vec();
        vec.sort();
        Self(vec)
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn rule_out(&self, with: &Clause, target: &Literal) -> Self {
        let mut vec = Vec::with_capacity(self.0.len() - 1);
        for literal in self.0.iter().chain(with.0.iter()) {
            if literal.m != target.m {
                vec.push(*literal);
            }
        }
        vec.sort();
        Self(vec)
    }
}

#[derive(Clone, Debug)]
struct Statement(HashSet<Clause>);

impl Statement {
    fn new(clauses: &[Clause]) -> Self {
        Self(clauses.iter().cloned().collect())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn insert(&mut self, clause: Clause) -> bool {
        self.0.insert(clause)
    }

    fn resolve(&self) -> bool {
        let mut knowledge_base = self.clone();
        let kb_size = knowledge_base.len();
        for clause1 in &self.0 {
            for literal1 in &clause1.0 {
                for clause2 in &self.0 {
                    for literal2 in &clause2.0 {
                        if literal1.m == literal2.m && literal1.negated != literal2.negated {
                            eprint!(
                                "Ruling out {} from {} and {} ",
                                literal1.normalize(),
                                clause1,
                                clause2
                            );
                            let new_clause = clause1.rule_out(&clause2, literal1);
                            eprint!("producing {}", new_clause);
                            if new_clause.is_empty() {
                                eprintln!(" which proves the statement unsatisfiable.");
                                return true;
                            }
                            if knowledge_base.insert(new_clause) {
                                eprintln!(" which is new knowledge.")
                            } else {
                                eprintln!(" which is redundant knowledge.");
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

fn main() {
    let statement = Statement::new(&[
        Clause::new(&[
            Literal {
                m: 1,
                negated: false,
            },
            Literal {
                m: 3,
                negated: false,
            },
        ]),
        Clause::new(&[
            Literal {
                m: 1,
                negated: true,
            },
            Literal {
                m: 2,
                negated: false,
            },
        ]),
        Clause::new(&[
            Literal {
                m: 2,
                negated: true,
            },
            Literal {
                m: 3,
                negated: false,
            },
        ]),
        Clause::new(&[Literal {
            m: 3,
            negated: true,
        }]),
    ]);

    println!("{:?}", statement.resolve());
}
