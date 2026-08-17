use crate::PixelUpdate;

#[derive(Debug, Clone, Copy)]
pub enum BroadcastEvent {
    PixelUpdated(PixelUpdate),
    SoldierCountChanged(usize),
}
