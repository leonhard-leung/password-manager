mod manager;
fn main() {
    manager::add("MyPotato".to_string(), "fries".to_string(), "12345".to_string(), "a dummy sample".to_string());

    manager::display()
}
