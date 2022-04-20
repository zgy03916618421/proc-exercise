use my_derive::MySerialize;

#[derive(MySerialize)]
struct User {
    age: String,
    city: String,
}

fn main() {
    let a = User {
        age: String::from("zhougy"),
        city: String::from("guangzhou"),
    };

    a.serialize();
}
