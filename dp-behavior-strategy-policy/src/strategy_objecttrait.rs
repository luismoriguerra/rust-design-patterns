use std::collections::HashMap;

type Data = HashMap<String, i32>;

trait Formatter {
    fn format(&self, data: &Data, s: &mut String);
}

struct Report;
impl Report {
    fn generate<T: Formatter>(g: T, s: &mut String) {
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);

        g.format(&data, s)
    }
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{} {}\n", k, v);
            buf.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop();
        }
        buf.push(']');
    }
}

pub fn main_example() {
    let mut s = String::from("");

    Report::generate(Text, &mut s);
    println!("{}", s);
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    s.clear();
    Report::generate(Json, &mut s);
    println!("{}", s);
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
}
