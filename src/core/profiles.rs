use duration_str::HumanFormat;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Add;
use std::process;

use std::fmt::Display;
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Importance {
    VeryHigh,
    High,
    Middle,
    Low,
    Zero,
}

impl Importance {
    pub fn new(importance: usize) -> Self {
        match importance {
            0 => Importance::Zero,
            1 => Importance::Low,
            2 => Importance::Middle,
            3 => Importance::High,
            _ => Importance::VeryHigh,
        }
    }
}

impl Display for Importance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text;

        match self {
            Importance::VeryHigh => text = "A",
            Importance::High => text = "B",
            Importance::Middle => text = "C",
            Importance::Low => text = "D",
            Importance::Zero => text = "E",
        }

        write!(f, "{text}")
    }
}

#[derive(PartialEq, Debug, Serialize, Clone, Deserialize)]
enum TaskTimerType {
    Reccurence,
    Expire,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
struct TaskTimer {
    timer_type: TaskTimerType,
    next: SystemTime,
    duration: Duration,
}

impl TaskTimer {
    fn new(timer_type: TaskTimerType, duration: Duration) -> Self {
        Self {
            timer_type,
            next: SystemTime::now().add(duration),
            duration,
        }
    }

    fn human_format(&self) -> String {
        self.next
            .duration_since(SystemTime::now())
            .unwrap()
            .human_format()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .drain(0..2)
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ToDo {
    field: String,
    complete: bool,
    importance: Importance,
    task_timer: Option<TaskTimer>,
}

impl ToDo {
    fn new(s: &str, importance: usize, task_timer: Option<TaskTimer>) -> ToDo {
        Self {
            field: s.to_string(),
            complete: false,
            importance: Importance::new(importance),
            task_timer,
        }
    }

    fn update(&mut self) {
        println!("UPDATE CALLED");
        if let Some(timer) = &mut self.task_timer {
            if timer.timer_type == TaskTimerType::Expire {
                timer.next = SystemTime::now() + timer.duration;
            } else {
                if SystemTime::now() > timer.next {
                    self.complete = false;
                }
                while SystemTime::now() > timer.next {
                    timer.next += timer.duration;
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub content: Vec<ToDo>,
}

impl Profile {
    pub fn update(&mut self) {
        for i in &mut self.content {
            if let Some(timer) = &mut i.task_timer {
                if timer.timer_type == TaskTimerType::Reccurence {
                    i.update();
                }
            }
        }
    }

    pub fn list(&self, show_importance: bool, numerate_from_zero: bool) {
        let mut str = format!("Profile '{}':\n\n", self.name);
        let mut num = if numerate_from_zero { 0 } else { 1 };

        for i in &self.content {
            let buf = if i.complete {
                "[ X ]".to_string()
            } else {
                "[   ]".to_string()
            };

            let buf = if show_importance {
                match &i.task_timer {
                    Some(t) => match t.timer_type {
                        TaskTimerType::Expire => {
                            if SystemTime::now() > t.next {
                                format!("{num}. {} | Expired {buf} {}\n", i.importance, i.field)
                            } else {
                                format!(
                                    "{num}. {} | Expires in: {} {buf} {}\n",
                                    i.importance,
                                    t.human_format(),
                                    i.field
                                )
                            }
                        }
                        TaskTimerType::Reccurence => {
                            format!(
                                "{num}. {} | Resets in: {} {buf} {}\n",
                                i.importance,
                                t.human_format(),
                                i.field
                            )
                        }
                    },

                    None => format!("{num}. {} {buf} {}\n", i.importance, i.field),
                }
            } else {
                match &i.task_timer {
                    Some(t) => match t.timer_type {
                        TaskTimerType::Expire => {
                            if SystemTime::now() > t.next {
                                format!("{num}. Expired {buf} {}\n", i.field)
                            } else {
                                format!(
                                    "{num}. Expires in: {} {buf} {}\n",
                                    t.human_format(),
                                    i.field
                                )
                            }
                        }
                        TaskTimerType::Reccurence => {
                            format!("{num}. Resets in: {} {buf} {}\n", t.human_format(), i.field)
                        }
                    },

                    None => format!("{num}. {buf} {}\n", i.field),
                }
            };

            str.push_str(&buf);

            num += 1;
        }

        println!("{str}");
    }

    pub fn add(
        &mut self,
        text: &str,
        importance: usize,
        reccurence: Option<String>,
        expire: Option<String>,
    ) {
        if reccurence.is_some() && expire.is_some() {
            eprintln!("The task cannot have a reccurence and expire");
            process::exit(1);
        }

        let task_timer = if let Some(expire) = expire {
            match duration_str::parse(expire) {
                Ok(d) => Some(TaskTimer::new(TaskTimerType::Expire, d)),
                Err(e) => {
                    eprintln!("Cannot parse duration from task expire: {e}");
                    process::exit(1);
                }
            }
        } else if let Some(reccurence) = reccurence {
            match duration_str::parse(reccurence) {
                Ok(d) => Some(TaskTimer::new(TaskTimerType::Reccurence, d)),
                Err(e) => {
                    eprintln!("Cannot parse duration from task expire: {e}");
                    process::exit(1);
                }
            }
        } else {
            None
        };

        self.content.push(ToDo::new(text, importance, task_timer));
    }

    pub fn edit(
        &mut self,
        target: usize,
        text: Option<String>,
        importance: Option<usize>,
        numerate_from_zero: bool,
    ) {
        let target = if numerate_from_zero {
            target
        } else {
            target - 1
        };

        match self.content.get_mut(target) {
            Some(t) => {
                if let Some(nt) = text {
                    t.field = nt;
                }

                if let Some(i) = importance {
                    t.importance = Importance::new(i);
                }
            }
            None => eprintln!("Can not find a task with this number"),
        }
    }

    pub fn importance(&mut self, target: usize, importance: usize, numerate_from_zero: bool) {
        let target = if numerate_from_zero {
            target
        } else {
            target - 1
        };

        let target = match self.content.get_mut(target) {
            Some(t) => t,
            None => return,
        };

        let importance = Importance::new(importance);

        target.importance = importance;
    }

    pub fn remove(&mut self, target: usize, numerate_from_zero: bool) {
        let target = if numerate_from_zero {
            target
        } else {
            target - 1
        };

        match self.content.get(target) {
            Some(_) => (),
            None => {
                eprintln!("Can not find a task with this number");
                process::exit(1);
            }
        }

        self.content.remove(target);
    }

    pub fn remove_completed(&mut self) {
        self.content.retain(|x| !x.complete);
    }

    pub fn complete(&mut self, target: usize, numerate_from_zero: bool) {
        let target = if numerate_from_zero {
            target
        } else {
            target - 1
        };

        let target = match self.content.get_mut(target) {
            Some(t) => t,
            None => {
                eprintln!("Can not find a task with this number");
                return;
            }
        };

        target.complete = !target.complete;
    }

    pub fn clear(&mut self) {
        self.content = vec![];
    }

    pub fn complete_all(&mut self) {
        self.content.iter_mut().for_each(|t| t.complete = true);
    }

    pub fn decomplete_all(&mut self) {
        self.content.iter_mut().for_each(|t| t.complete = false);
    }

    pub fn change_all(&mut self) {
        self.content
            .iter_mut()
            .for_each(|t| t.complete = !t.complete);
    }

    // Self:
    pub fn new(profile_name: &str) -> Self {
        Self {
            name: profile_name.to_string(),
            content: vec![],
        }
    }
}
