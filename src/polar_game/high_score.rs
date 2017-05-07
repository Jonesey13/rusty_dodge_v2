use time;
use rendering::PlainText;
use na::{Vector2, Vector4, Matrix4, Rotation2};
use std::fs::File;
use std::string::String;
use std::io::{Read, Write};

pub struct HighScore {
    current_score: f64,
    record: f64
}

impl HighScore {
    pub fn new() -> HighScore {
        HighScore {
            current_score: 0.0,
            record: HighScore::load_high_score()
        }
    }

    pub fn update(&mut self, t_step: f64) {
        self.current_score += t_step;
        if self.current_score > self.record {
            self.record = self.current_score;
        }
    }

    pub fn reset(&mut self) {
        self.current_score = 0.0;
    }

    fn get_score_string(&self) -> String {
        format!("Current Score: {0:.2}", self.current_score)
    }

    fn get_record_string(&self) -> String {
        format!("Record: {0:.2}", self.record)
    }

    pub fn get_score_text(&self) -> PlainText {
        PlainText {
            content: self.get_score_string(),
            position: Vector2::new(0.8, 0.9),
            scale: Vector2::new(1.0, 1.0),
            transform: *Rotation2::new(0.0).matrix(),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            fixed: true
        }
    }

    pub fn get_record_text(&self) -> PlainText {
        PlainText {
            content: self.get_record_string(),
            position: Vector2::new(0.8, 0.8),
            scale: Vector2::new(1.0, 1.0),
            transform: *Rotation2::new(0.0).matrix(),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            fixed: true
        }
    }

    pub fn load_high_score() -> f64 {
        let path = "highscore.txt".to_string();
        let mut file = match File::open(path){
            Ok(f) => f,
            Err(_) => return 0.0,
        };
        let mut score_string = String::new();
        match file.read_to_string(&mut score_string){
            Ok(r) => r,
            Err(_) => panic!("failed to read score to string"),
        };
        match score_string.parse::<f64>(){
            Ok(s) => s,
            Err(_) => panic!("failed to parse string to f64"),
        }
    }

    pub fn update_high_score(&self) {
        if self.current_score < self.record { return; } 
        let score_string = self.current_score.to_string();
        let path = "highscore.txt".to_string();
        let mut file = match File::create(path){
            Ok(f) => f,
            Err(_) => panic!("Failed to open High Score!"),
        };
        match file.write_all(score_string.as_bytes()){
            Ok(f) => f,
            Err(_) => panic!("failed to write sting to file"),
        };
    }
}
