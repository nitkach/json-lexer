use askama::Template;

pub const MARES_COUNT: usize = 1_000_000;

#[derive(Debug, Template)]
#[template(path = "hello.html")]
// #[template(source = "Hello, {{ name }}!", ext = "txt")]
struct HelloTemplate {
    name: String,
}

#[derive(Debug, Template)]
#[template(path = "child.html")]
struct BaseChildTemplate;

#[derive(Debug, Template)]
#[template(path = "control.html")]
struct ControlTemplate {
    mares: Vec<String>,
}

#[derive(Debug, Template)]
#[template(path = "include.html")]
struct IncludeTemplate {
    foo: String,
}

#[derive(Debug, Template)]
#[template(source = "{{ s1 }}", ext = "txt")]
struct RenderInPlaceTemplate {
    s1: SectionOneTemplate,
}

#[derive(Debug, Template)]
#[template(
    source = "+++ source +++\na = {{ a }}\nb = {{ b }}\na + b = {{ a }} {{ b }}\n--- source ---",
    ext = "txt"
)]
struct SectionOneTemplate {
    a: String,
    b: String,
}

#[derive(Debug, Template)]
#[template(path = "filters.html", escape = "none")]
struct FiltersTemplate {
    foo: String,
}

mod filters {
    use itertools::Itertools;
    use std::fmt::Display;

    pub fn marelling(to_mare: impl Display) -> askama::Result<String> {
        Ok(to_mare
            .to_string()
            .split(' ')
            .map(|string| {
                let mut mare_str = "mare".to_owned();
                if string.len() > 4 {
                    let (_, right_part) = string.split_at(4);
                    mare_str.push_str(right_part);
                } else {
                    mare_str.push_str(&format!("'{string}"));
                }
                mare_str
            })
            .join(" "))
    }
}

fn main() {
    let hello = HelloTemplate {
        name: "mares".to_owned(),
    };
    println!("{}", hello.render().unwrap());

    let child = BaseChildTemplate;
    println!("{}", child.render().unwrap());

    let control = ControlTemplate {
        mares: vec![
            "Minuette".to_owned(),
            "Lemon Hearts".to_owned(),
            "Twinkleshine".to_owned(),
        ],
    };
    println!("{}", control.render().unwrap());

    let include = IncludeTemplate {
        foo: "mares are so pretty!".to_owned(),
    };
    println!("{}", include.render().unwrap());

    let render_in_place = RenderInPlaceTemplate {
        s1: SectionOneTemplate {
            a: "mares".to_owned(),
            b: "are nice!".to_owned(),
        },
    };
    println!("{}", render_in_place.render().unwrap());

    let filters = FiltersTemplate { foo: r#"To define your own filters, simply have a module named filters in scope of the context deriving a Template impl and define the filters as functions within this module. The functions must have at least one argument and the return type must be ::askama::Result<T>. Although there are no restrictions on T for a single filter, the final result of a chain of filters must implement Display."#.to_owned() };
    println!("{}", filters.render().unwrap());
}
