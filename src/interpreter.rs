use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::{Write, empty, stdout};

use crate::ast::deep_clone::NodeDeepClone;
use crate::ast::{Node, assignments::insert_many_assignments, expr::Unzipped};
use crate::cli::LogLevel::{AssignmentReplacements, ReductionsOnly};
use crate::stdlib::stdlib_assignments;
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

#[derive(Debug)]
struct NodeHist {
    original: NodeDeepClone,
    assignment_subs: Vec<NodeDeepClone>,
    reductions: Vec<NodeDeepClone>,
}

impl NodeHist {
    pub fn new(original: NodeDeepClone) -> Self {
        Self {
            original,
            assignment_subs: Vec::new(),
            reductions: Vec::new()
        }
    }
}

impl Display for NodeHist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeHist")
            .field("original", &self.original.to_string())
            .field(
                "assignment_sub",
                &self.assignment_subs.iter().map(|s| s.to_string()),
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
        if !statements.assignments.is_empty() {
            writeln!(out, "{sep} Assignments {sep}", sep = "=".repeat(25))?;
        }

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
        let mut hist = NodeHist::new(Node::deep_clone(expression));

        loop {
            Node::substitute_assignments(expression, assignments, None);
            let clone = Node::deep_clone(expression);
            if hist
                .assignment_subs
                .last()
                .is_some_and(|last| *last == clone)
            {
                break;
            }

            hist.assignment_subs.push(clone);
        }

        loop {
            Node::reduce(expression, None, None);
            let clone = Node::deep_clone(expression);
            if Node::has_cycle(expression, &mut HashSet::new()) {
                panic!("Cycle found in expression! Expression: {}", expression);
            }

            if hist
                .reductions
                .last()
                .is_some_and(|last| *last == clone)
            {
                break;
            }

            hist.reductions.push(clone);
        }

        hists.push(hist);
    }

    Ok(hists)
}

pub fn interpret(input: &str, args: &CLIArgs) -> anyhow::Result<()> {
    let mut out = Out::new(args.stdout);
    let mut assignments = if args.nostdlib {
        HashMap::new()
    } else {
        stdlib_assignments()
    };

    let statements = Statement::parse(input)?.unzip();
    handle_assignments(&statements, &mut assignments, args, &mut out)?;

    let hists = handle_expressions(&statements, &assignments)?;

    for hist in hists {
        if args.loglevel >= LogLevel::All {
            writeln!(out, "{sep} Expressions {sep}", sep = "=".repeat(25))?;
            writeln!(out, "(Original)\n\t{:#}\n", hist.original)?;
        }

        if args.loglevel >= AssignmentReplacements && !hist.assignment_subs.is_empty() {
            writeln!(out, "(Assignment Replacements)")?;
            for sub in hist.assignment_subs {
                writeln!(out, "\t{:#}", sub)?;
            }

            writeln!(out)?;
        }

        if args.loglevel >= ReductionsOnly {
            writeln!(out, "(Reductions):")?;
            for reduction in hist.reductions {
                writeln!(out, "     => {}", reduction)?;
            }
        }
    }

    writeln!(out)?;
    out.flush()?;

    Ok(())
}
