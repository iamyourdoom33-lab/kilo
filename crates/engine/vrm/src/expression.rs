//! Blend shape expressions (joy, anger, sadness, surprise).

//! Maps VRM blend shapes to character emotions

pub enum Expression {
    Joy,
    Anger,
    Sadness,
    Surprise,
    Neutral,
}

pub struct ExpressionPlugin;

impl bevy::app::Plugin for ExpressionPlugin {
    fn build(&self, _app: &mut bevy::app::App) {}
}
