mod strategy_objecttrait;

struct Adder;
impl Adder {
    pub fn add<F>(x: u8, y: u8, f: F) -> u8
    where
        F: Fn(u8, u8) -> u8,
    {
        f(x, y)
    }
}

fn main() {
    strategy_objecttrait::main_example();

    // Strategy pattern with closures
    {
        let arith_adder = |x, y| x + y;

        assert_eq!(Adder::add(1, 2, arith_adder), 3);

        let bool_adder = |x, y| {
            if x == 1 || y == 1 {
                1
            } else {
                0
            }
        };

        assert_eq!(Adder::add(1, 0, bool_adder), 1);

        let custom_adder = |x, y| 2 * x + y;

        assert_eq!(Adder::add(1, 2, custom_adder), 4);
    }

    // strategy with Options map

    {
        let val = Some("Rust");

        let len_strategy = |s: &str| s.len();

        assert_eq!(4, val.map(len_strategy).unwrap());

        let first_byte_strategy = |s: &str| s.bytes().next().unwrap();
        assert_eq!(82, val.map(first_byte_strategy).unwrap());
    }
}
