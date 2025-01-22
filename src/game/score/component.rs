use bevy:: prelude:: Resource; 

#[derive(Resource)]
#[derive(Default)]
pub struct Score {
   pub value: u32
}

#[derive(Resource)]
#[derive(Default)]
pub struct HighScore {
    pub scores: Vec<(String, u32)>
}


