mod ast {
    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }

    pub enum Stmt {
        Expr(Expr),
        Let(String, Expr),
    }

    pub struct Name {
        value: String,
    }
}

mod visit {
    use crate::ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}

use crate::ast::*;
use crate::visit::*;

struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 {
        panic!()
    }

    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref e1, ref e2) => self.visit_expr(e1) + self.visit_expr(e2),
            Expr::Sub(ref e1, ref e2) => self.visit_expr(e1) - self.visit_expr(e2),
        }
    }
}

impl Interpreter {
    pub fn walk_expr(visitor: &mut dyn Visitor<i32>, e: &Expr) {
        match *e {
            Expr::IntLit(_) => {}
            Expr::Add(ref lhs, ref rhs) => {
                visitor.visit_expr(lhs);
                visitor.visit_expr(rhs);
            }
            Expr::Sub(ref lhs, ref rhs) => {
                visitor.visit_expr(lhs);
                visitor.visit_expr(rhs);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
