use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ModelType {
    pub side: ModelSide,
    pub action: ModelAction,
}

impl TryFrom<usize> for ModelType {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= Self::N_VARIANTS {
            Err(anyhow!("ModelType index out of bounds"))
        } else {
            let side = unsafe { std::mem::transmute((value / 2) as u32) };
            let action = unsafe { std::mem::transmute((value % 2) as u32) };
            Ok(Self { side, action })
        }
    }
}

impl Into<usize> for ModelType {
    fn into(self) -> usize {
        let side: usize = self.side as usize;
        let action: usize = self.action as usize;
        side * 2 + action
    }
}

impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action_str = format!("{:?}", self.action).to_lowercase();
        let side_str = format!("{:?}", self.side).to_lowercase();
        write!(f, "{}_{}", action_str, side_str)
    }
}

impl ModelType {
    pub const N_VARIANTS: usize = 4;

    pub fn all() -> Vec<Self> {
        (0..Self::N_VARIANTS)
            .map(|i| Self::try_from(i).unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ModelSide {
    Long,
    Short,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ModelAction {
    Opening,
    Closing,
}
