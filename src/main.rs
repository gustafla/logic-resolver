use resolution::{Statement, Clause, Literal};

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

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
