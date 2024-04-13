mod comandtraitobject;
mod function_pointer;

type Migration<'a> = Box<dyn Fn() -> &'a str>;

struct Schema<'a> {
    executes: Vec<Migration<'a>>,
    rollbacks: Vec<Migration<'a>>,
}

impl<'a> Schema<'a> {
    fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![],
        }
    }

    fn add_migration<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static,
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }

    fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

fn add_field() -> &'static str {
    "add field"
}
fn remove_field() -> &'static str {
    "remove field"
}

fn main() {
    comandtraitobject::main_example();
    function_pointer::main_example();
    println!("Design pattern Behavior Command using Fn Trait objects: advanced");

    let mut scehma = Schema::new();
    let fncreatetable = || "create table";
    let fndroptable = || "drop table";

    scehma.add_migration(fncreatetable, fndroptable);

    scehma.add_migration(add_field, remove_field);

    assert_eq!(vec!["create table", "add field"], scehma.execute());
    assert_eq!(vec!["remove field", "drop table"], scehma.rollback());
}
