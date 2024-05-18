use super::*;

pub type Idx = (usize, usize);

#[derive(Component, DerefMut, Deref)]
pub struct Hint(pub Idx);

#[derive(Component, DerefMut, Deref)]
pub struct HorizontalFence(pub Idx);

#[derive(Component, DerefMut, Deref)]
pub struct VerticalFence(pub Idx);
