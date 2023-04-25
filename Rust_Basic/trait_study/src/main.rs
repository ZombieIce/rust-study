pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

// impl Post {
//     fn summarize(&self) -> String {
//         format!("articel {}, author {}", self.title, self.author)
//     }
// }

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} post a weibo {}", self.username, self.content)
//     }
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let post = Post {
        title: "Rust Brief".to_string(),
        author: "Sunface".to_string(),
        content: "awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "not good enough as tweet".to_string(),
    };

    println!("{}", post.summarize());
    // println!("{}", weibo.summarize());

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
