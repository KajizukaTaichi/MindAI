use std::io::{self, Write};

/// 入力
fn input(prompt: &str) -> String {
    print!("{}", prompt.to_string());
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    return result.trim().parse().ok().unwrap();
}

fn main() {
    let mut brain = Brain::new();
    loop {
        brain.communication()
    }
}

/////////////////////////////
/// 心理学的に人間の営みを再現 ////
////////////////////////////

/// 論理
#[derive(Debug, Clone)]
struct Logic {
    word: String,
    mean: String,
    emotion: Emotion,
}

/// 感情
#[derive(Debug, Copy, Clone, PartialEq)]
enum Emotion {
    Happy(usize),
    Angry(usize),
    Sad(usize),
    Normal(usize),
}

/// 能力
trait Ability {
    fn new() -> Brain;
    fn communication(&mut self);
    fn remember(&mut self, word: String) -> Logic;
    fn study(&mut self, word: String);
}

/// 脳
struct Brain {
    memory: Vec<Logic>,
    emotion: Emotion,
}

impl Ability for Brain {
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
        }
    }

    /// 記憶を思い出す
    fn remember(&mut self, word: String) -> Logic {
        for item in self.memory.clone() {
            if word.contains(&item.word) {
                match (self.emotion, item.emotion) {
                    (Emotion::Happy(i), Emotion::Happy(j)) => {self.emotion =  Emotion::Happy(i + j)},
                    (Emotion::Angry(i), Emotion::Angry(j)) => {self.emotion =  Emotion::Angry(i + j)},
                    (Emotion::Sad(i), Emotion::Sad(j)) => {self.emotion =  Emotion::Sad(i + j)},
                    _ => self.emotion = item.emotion
                };

                return item;
            }
        }
        self.study(word.clone());
        self.remember(word)
    }

    /// 学習
    fn study(&mut self, word: String) {
        let mean = input("どういう意味なの？");
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
        })
    }

    /// ユーザーとやりとり
    fn communication(&mut self) {
        let msg: String = input("> ");
        let item = self.remember(msg);

        // 感情を表すメッセーッジ
        let emo_msg = match &self.emotion {
            Emotion::Happy(_) => "ありがとう！",
            Emotion::Angry(_) => "何なのよっ",
            Emotion::Sad(_) => "ふん・・",
            Emotion::Normal(_) => "そうか。",
        };
        println!("感情:{:?} {emo_msg} 私、{}の？", self.emotion, item.mean);
        return;
    }
}
