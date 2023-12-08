use std::io::{self, Write};

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

#[derive(Debug, Clone)]
struct Logic {
    word: String,
    mean: String,
    emotion: Emotion,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Emotion {
    Happy(usize),
    Angry(usize),
    Sad(usize),
    Normal(usize),
}

impl Emotion {
    fn get_value(&self) -> usize {
        match &self {
            Emotion::Happy(i) => *i,
            Emotion::Angry(i) => *i,
            Emotion::Sad(i) => *i,
            Emotion::Normal(i) => *i,
        }
    }
}

struct Brain {
    memory: Vec<Logic>,
    emotion: Emotion, // 自分のへ気持ち
    liking: usize,    // ユーザーへの好感度
    belief: Logic,    // 自分の信念
}

impl Brain {
    fn new() -> Brain {
        Brain {
            memory: vec![
                Logic {
                    word: "バカ".to_string(),
                    mean: "頭が悪い".to_string(),
                    emotion: Emotion::Angry(85),
                },
                Logic {
                    word: "天才".to_string(),
                    mean: "優れている".to_string(),
                    emotion: Emotion::Happy(100),
                },
            ],
            emotion: Emotion::Normal(0),
            belief: Logic {
                word: "天才".to_string(),
                mean: "優れている".to_string(),
                emotion: Emotion::Happy(100),
            }, // 自分の信念を初期化
            liking: 0, // 好感度を初期化
        }
    }

    fn update_bias(&mut self, user_belief: &str) {
        if user_belief.contains(&self.belief.word) {
            match &mut self.emotion {
                Emotion::Happy(i) => *i += 50,
                Emotion::Angry(i) if *i >= 50 => *i -= 50,
                _ => {}
            };
        } else {
            match &mut self.emotion {
                Emotion::Happy(i) if *i >= 50 => *i -= 50,
                Emotion::Angry(i) => *i += 50,
                _ => {}
            };
        }
    }

    // 考える
    fn think(&mut self, word: String, mean: String) -> Option<Logic> {
        for item in &self.memory {
            if mean.contains(&item.word) {
                return Some(Logic {
                    word,
                    ..item.clone()
                });
            }
        }
        None
    }

    fn remember(&mut self, word: String) -> Logic {
        for item in &self.memory {
            if word.contains(&item.word) {
                // 好感度の計算と更新
                match &mut self.emotion {
                    Emotion::Happy(i) => {
                        self.liking += 10;
                        *i += item.emotion.get_value() / self.liking;
                    }
                    Emotion::Angry(i) => {
                        self.liking -= 5;
                        *i += item.emotion.get_value() / self.liking;
                    }
                    Emotion::Sad(i) => {
                        self.liking -= 7;
                        *i += item.emotion.get_value() / self.liking;
                    }
                    _ => {
                        self.emotion = item.emotion;
                    }
                };

                return item.clone();
            }
        }
        self.study(word.clone());
        self.remember(word)
    }

    fn study(&mut self, word: String) {
        let mean = input("どういう意味なの？");
        let result: Option<Logic> = self.think(word.clone(), mean.clone());
        match result {
            Some(i) => self.memory.push(i),
            None => {
                let binding = input("どういう感情なの？");
                let emo = binding.as_str();
                let st = input("どんくらい刺激があるの？").parse().unwrap_or(50);
                self.memory.push(Logic {
                    word,
                    mean,
                    emotion: match emo {
                        "幸せ" => Emotion::Happy(st),
                        "怒り" => Emotion::Angry(st),
                        "悲しい" => Emotion::Sad(st),
                        _ => Emotion::Normal(st),
                    },
                });
            }
        }
    }

    fn communication(&mut self) {
        let msg: String = input("> ").trim().to_string();
        let (subject, msg): (String, String) = match msg.split_once("は") {
            Some((s, m)) => (s.trim().to_string(), m.trim().to_string()),
            None => ("".to_string(), msg),
        };
        self.update_bias(&msg);
        let item = self.remember(msg);

        if subject.contains("俺") || subject.contains("私") || subject.contains("ワイ") {
            if &self.liking > &0 {
                match &self.emotion {
                    Emotion::Happy(i) => {
                        println!(
                            "感情: 幸せ{} {{ やったあ！ あなたも{}なんだぜ！",
                            i, item.mean
                        )
                    }
                    Emotion::Angry(i) => {
                        println!("感情: 怒り{} {{ あなたが{}っていうの？！", i, item.mean)
                    }
                    Emotion::Sad(i) => println!(
                        "感情: 悲しさ{} {{ ふん・・ あなたは{}だったのかい？",
                        i, item.mean
                    ),
                    Emotion::Normal(i) => {
                        println!("感情: ふつう{} {{ そうか。 あなたは{}んだね", i, item.mean)
                    }
                };
            } else {
                match &self.emotion {
                    Emotion::Angry(i) => {
                        println!(
                            "感情: 幸せ{} {{ やったあ！ あなたは{}なんだぜ！",
                            i, item.mean
                        )
                    }
                    Emotion::Happy(i) => {
                        println!("感情: 怒り{} {{ なんであなたが{}なんか・・。", i, item.mean)
                    }
                    Emotion::Sad(i) => {
                        println!("感情: 幸せ{} {{ あなたは{}だって？", i, item.mean)
                    }
                    Emotion::Normal(i) => {
                        println!("感情: ふつう{} {{ そうか。 あなたは{}んだね", i, item.mean)
                    }
                };
            }
        } else {
            match &self.emotion {
                Emotion::Happy(i) => {
                    println!("感情: 幸せ{} {{ やったあ！ 私は{}なんだぜ！", i, item.mean)
                }
                Emotion::Angry(i) => println!(
                    "感情: 怒り{} {{ 何なのよっ 私が{}っていうの？！",
                    i, item.mean
                ),
                Emotion::Sad(i) => println!(
                    "感情: 悲しさ{} {{ ふん・・ 私は{}だってのかい？",
                    i, item.mean
                ),
                Emotion::Normal(i) => {
                    println!("感情: ふつう{} {{ そうか。 私は{}んだね", i, item.mean)
                }
            };
        }
    }
}

fn main() {
    let mut brain = Brain::new();
    loop {
        brain.communication()
    }
}
