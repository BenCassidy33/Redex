use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Write, empty, stdout};

use derive_more::Constructor;

use crate::ast::Node;
use crate::ast::assignments::insert_many_assignments;
use crate::ast::expr::Unzipped;
use crate::cli::LogLevel::{AssignmentReplacements, ReductionsOnly};
use crate::{
    ast::{
        assignments::Assignments,
        expr::{Statement, Unzip},
    },
    cli::{CLIArgs, LogLevel},
};

struct Out {
    buf: Vec<u8>,
    stdout: Box<dyn Write>,
}

impl Out {
    pub fn new(print_to_stdout: bool) -> Self {
        Self {
            buf: Vec::new(),
            stdout: if print_to_stdout {
                Box::new(stdout().lock())
            } else {
                Box::new(empty())
            },
        }
    }
}

impl Write for Out {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let vw = self.buf.write(buf)?;
        let ow = self.stdout.write(buf)?;

        Ok(vw.min(ow))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf.flush()?;
        self.stdout.flush()
    }
}

#[derive(Debug, Constructor)]
struct NodeHist {
    original: Node,
    assignment_sub: Option<Node>,
    reductions: Vec<Node>,
}

impl Display for NodeHist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeHist")
            .field("original", &self.original.to_string())
            .field(
                "assignment_sub",
                &self.assignment_sub.as_ref().map(|s| s.to_string()),
            )
            .field("reductions", &self.reductions)
            .finish()
    }
}

#[inline]
fn handle_assignments(
    statements: &Unzipped,
    assignments: &mut Assignments,
    args: &CLIArgs,
    out: &mut Out,
) -> anyhow::Result<()> {
    if args.loglevel >= LogLevel::Assignment {
        writeln!(out, "{sep} Assignments {sep}", sep = "=".repeat(25))?;

        for assignment in &statements.assignments {
            writeln!(out, "{}", assignment)?;
        }
    }

    insert_many_assignments(assignments, statements.assignments.clone());
    Ok(())
}

#[inline]
fn handle_expressions(
    statements: &Unzipped,
    assignments: &Assignments,
) -> anyhow::Result<Vec<NodeHist>> {
    let mut hists: Vec<NodeHist> = Vec::new();
    for expression in &statements.expressions {
        let mut hist = NodeHist::new(expression.deep_clone(), None, Vec::new());
        Node::substitute_assignments(expression, assignments, None);

        if *expression != hist.original {
            hist.assignment_sub = Some(expression.deep_clone());
        }

        while hist
            .reductions
            .last()
            .is_none_or(|last| *last != *expression.borrow())
        {
            Node::reduce(expression, None, None);
            hist.reductions.push(expression.deep_clone());
        }

        hists.push(hist);
    }

    Ok(hists)
}

pub fn interpret(input: &str, args: &CLIArgs) -> anyhow::Result<()> {
    let mut out = Out::new(args.stdout);
    let mut assignments: Assignments = HashMap::new();

    let statements = Statement::parse(input)?.unzip();
    handle_assignments(&statements, &mut assignments, args, &mut out)?;

    let hists = handle_expressions(&statements, &assignments)?;

    for hist in hists {
        if args.loglevel >= LogLevel::All {
            writeln!(out, "{sep} Expressions {sep}", sep = "=".repeat(25))?;
            writeln!(out, "(Original)                 {:#}", hist.original)?;
        }

        if args.loglevel >= AssignmentReplacements
            && let Some(sub) = hist.assignment_sub
        {
            writeln!(out, "(Assignment Replacements)  {:#}", sub)?;
        }

        if args.loglevel >= ReductionsOnly {
            writeln!(out, "(Reductions):\n{}", hist.original)?;
            for reduction in hist.reductions {
                writeln!(out, "=> {}", reduction)?;
            }
        }
    }

    writeln!(out)?;
    out.flush()?;

    Ok(())
}
