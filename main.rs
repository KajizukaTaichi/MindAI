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
struct Logic {
    word: String,
    mean: String,
    emotion: Emotion,
    stimulation: u8,
}

/// 感情
#[derive(Debug, Copy, Clone)]
enum Emotion {
    Happy,
    Angry,
    Sad,
    Normal,
}

/// 能力
trait Ability {
    fn new() -> Brain;
    fn communication(&mut self);
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
                    emotion: Emotion::Angry,
                    stimulation: 100,
                },
                Logic {
                    word: "天才".to_string(),
                    mean: "優れている".to_string(),
                    emotion: Emotion::Happy,
                    stimulation: 100,
                },
            ],
            emotion: Emotion::Normal,
        }
    }

    fn communication(&mut self) {
        let msg: String = input("> ");

        // 記憶を思い出す
        for item in &self.memory {
            if msg.contains(&item.word) {
                self.emotion = item.emotion;

                // 感情を表すメッセーッジ
                let emo_msg = match &self.emotion {
                    Emotion::Happy => "ありがとう！",
                    Emotion::Angry => "何なのよっ",
                    Emotion::Sad => "ふん・・",
                    Emotion::Normal => "そうか。",
                };
                if item.stimulation >= 50 {
                    println!("{emo_msg} 私、{}の？", item.mean);
                }
                return;
            }
        }

        self.memory.push(Logic {
            word: msg,
            mean: input("どういう意味なの？"),
            emotion: match input("どういう感情なの？").as_str() {
                "幸せ" => Emotion::Happy,
                "怒り" => Emotion::Angry,
                "悲しい" => Emotion::Sad,
                _ => Emotion::Normal,
            },
            stimulation: input("どんくらい刺激があるの？").parse().unwrap_or(50),
        })
    }
}
