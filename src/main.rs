struct InfoContainer {
    name: String,
    content: Vec<InfoType>
}

enum InfoType {
    Text(String),
    Container(InfoContainer)
}

fn main() {

}