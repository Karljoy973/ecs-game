
use super::component::{HighScore, Score};

impl Default for Score {
    fn default() -> Score {
        Score { value : 0}
    }
}
impl Default for HighScore {
    fn default() -> HighScore {
        HighScore { scores : Vec::new()}
    }
}
