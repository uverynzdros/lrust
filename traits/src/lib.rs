pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}
