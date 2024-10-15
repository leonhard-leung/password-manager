mod manager;
fn main() {
    manager::add("sample3".to_string(), "MyPotato".to_string(), "fries".to_string(), "12345".to_string(), "a dummy sample".to_string());
    manager::add("sample4".to_string(), "Potatoes".to_string(), "fries".to_string(), "54321".to_string(), "another dummy sample".to_string());

    manager::display()
}
