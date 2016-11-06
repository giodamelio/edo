pub struct Edo {

}

impl Edo {
    pub fn new() -> Edo {
        Edo {
        }
    }

    fn parse(self) {
        println!("Hello World!");
    }
}

#[cfg(test)]
mod tests {
    use super::Edo;

    #[test]
    fn parse_template() {
        let parser = Edo::new();
        parser.parse();
    }
}
