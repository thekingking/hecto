use crate::editor::terminal::Position;

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    /// 将Location转换为Position
    fn from(location: Location) -> Self {
        Self {
            row: location.y,
            col: location.x,
        }
    }
} 

impl Location {
    /// 两个坐标的相对位置，求光标在view中的相对位置
    pub const fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    } 
}
