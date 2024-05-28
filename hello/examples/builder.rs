/*
 * @Author: plucky
 * @Date: 2023-04-14 08:57:33
 * @LastEditTime: 2023-06-26 18:04:06
 * @Description: 
 */
#![allow(dead_code)]

pub struct Task{
    pub title: String,
    pub done: bool,
    pub desc: Option<String>,
}

impl Task{
    pub fn new(title: impl Into<String>) -> Task{
        Task{
            title: title.into(),
            done: false,
            desc: None,
        }
    }
}

#[derive(Default, Clone)]
pub struct TaskBuilder{
    title: Option<String>,
    done: bool,
    desc: Option<String>,
}

impl TaskBuilder{
    pub fn new() -> TaskBuilder{
        TaskBuilder::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> TaskBuilder{
        _=self.title.insert(title.into());
        self
    }

    pub fn done(mut self, done: bool) -> TaskBuilder{
        self.done = done;
        self
    }

    pub fn desc(mut self, desc: impl Into<String>) -> TaskBuilder{
        _=self.desc.insert(desc.into());
        self
    }

    pub fn build(self) -> Task{
        Task{
            title: self.title.expect("title is required"),
            done: self.done,
            desc: self.desc,
        }
    }
}

pub fn main() {
    let task = TaskBuilder::new().title("Learn Rust").build();
    println!("task: {:?}", task.title);
}