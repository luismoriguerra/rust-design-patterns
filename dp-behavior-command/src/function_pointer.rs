type FnPtr = fn() -> String;
struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

struct Schema {
    commands: Vec<Command>,
}

impl Schema {
    fn new() -> Self {
        Schema { commands: vec![] }
    }

    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback });
    }

    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }

    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

pub fn main_example() {
    println!("Design pattern Behavior Command using function pointers");

    let mut schema = Schema::new();
    let fncreatetable = || "create table".to_string();
    let fndroptable = || "drop table".to_string();
    schema.add_migration(fncreatetable, fndroptable);

    assert_eq!(vec!["create table"], schema.execute());
    assert_eq!(vec!["drop table"], schema.rollback());
}
