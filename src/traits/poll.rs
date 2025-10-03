pub trait BestLanguage {
    fn language1(&self) -> String;
    fn language2(&self) -> String;
    fn language3(&self) -> String;
}

struct Languages {
    l1: String,
    l2: String,
    l3: String,
}

impl BestLanguage for Languages {
    fn language1(&self) -> String {
        format!("{} is the language of Choice", self.l1)
    }

    fn language2(&self) -> String {
        format!("{} is the language of Choice", self.l2)
    }

    fn language3(&self) -> String {
        format!("{} is the language of Choice", self.l3)
    }
}

fn poll(input: Vec<&str>) -> &str {
    let mut p = 0;
    let mut r = 0;
    let mut c = 0;

    for i in input {
        if i == "Python" {
            p += 1;
        };
        if i == "Rust" {
            r += 1;
        };
        if i == "C" {
            c += 1;
        }
    }

    let mut winner = "";
    if p > r && p > c {
        winner = "Python";
    } else if r > p && r > c {
        winner = "Rust";
    } else if c > p && c > r {
        winner = "C";
    } else {
        panic!("There is no winner!");
    }

    winner
}

fn r_winner(decision: &str) -> String {
    let l = Languages {
        l1: "Python".to_string(),
        l2: "Rust".to_string(),
        l3: "C".to_string(),
    };
    
    match decision {
        "Python" => l.language1(),
        "Rust" => l.language2(),
        "C" => l.language3(),
        _ => panic!("Language not found!"),
    }
}

fn main() {
    let poll_store: Vec<&str> = vec![
        "Python", "C", "C", "Python", "Rust", "Rust", "Rust", "C", "Rust",
        "Python", "Python", "Python", "C", "Rust", "C", "Rust", "Python", "C",
        "Rust", "Rust", "Rust",
    ];

    let winner = poll(poll_store);

    let test = r_winner(winner);
    
    println!("{}", test);
}