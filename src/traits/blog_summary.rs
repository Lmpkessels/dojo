use std::fmt;

pub trait Summary {
    fn summarize(&self) -> String;
    fn get_author(&self) -> String;
}

pub trait Display {
    fn likes(&self) -> String;
}

struct BlogPost {
    author: String,
    subject: String,
    header: String,
    date_published: String,
    likes: String,
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        format!("{} has written {} about {} and published it on {}", 
        self.author, self.header, self.subject, self.date_published)
    }

    fn get_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

impl Display for BlogPost {
    fn likes(&self) -> String {
        format!("{} likes", self.likes)
    }
}

fn summarize_post<T: Summary + Display>(item1: &T, item2: &T) 
-> (String, String) {
    (item1.summarize(), item2.likes())
}

fn a(item: &(impl Summary + Display)) -> (String, String) {
    (item.get_author(), item.likes())
}

fn return_summary() -> impl fmt::Display {
    let p = BlogPost {
        author: String::from("Naval"),
        subject: String::from("Systems"),
        header: String::from("Complex Systems Emerge From\
         Iterations On Simple Designs"),
        date_published: String::from("2025-10-01"),
        likes: String::from("323"),
    };

    println!("{:?}", p.likes());

    p.likes()
}

fn b<T, U>(t: &T, u: &U) -> Vec<String> 
where 
    T: Display + Summary,
    U: Summary + Display,
{
    let mut v: Vec<String> = Vec::new();
    v.push(t.likes());
    v.push(t.summarize());
    v.push(u.get_author());
    v.push(u.likes());
    
    v
}

fn main() {
    let post = BlogPost {
        author: String::from("Naval"),
        subject: String::from("Systems"),
        header: String::from("Complex Systems Emerge From\
         Iterations On Simple Designs"),
        date_published: String::from("2025-10-01"),
        likes: String::from("323"),
    };

    let post1 = BlogPost {
        author: String::from("Balaji"),
        subject: String::from("Property/Cryptography"),
        header: String::from("All Property Becomes Cryptography"),
        date_published: String::from("2025-06-28"),
        likes: String::from("1212")
    };
    
    let test = summarize_post(&post, &post1);
    println!("{:?}", test);

    let test0 = a(&post);
    println!("{:?}", test0);

    let test1 = b(&post, &post1);
    println!("{:?}", test1);

    return_summary();
}