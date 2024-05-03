pub mod user;

pub fn add_test(var: &str) -> String {
    let new_var = var.to_string() + "Test";

    let name = user::get_user("name".to_string());
    println!("{}", name);

    return new_var;
}
