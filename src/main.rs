struct InfoContainer {
    name: String,
    content: Vec<InfoType>
}

enum InfoType {
    Text(String),
    Container(InfoContainer)
}

fn main() {
    let inf = InfoContainer {name: String::from("Firkant"), content: vec![InfoType::Text(String::from("Hej"))]};

    let not_valid = &String::from("Der er mere");

    let out = match &inf.content[0] {
        InfoType::Text(s) => s,
        _ => {
            not_valid
        }
    };

    println!("{}", out)

}
