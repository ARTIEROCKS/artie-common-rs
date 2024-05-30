use serde::{Serialize, Deserialize};
use crate::structure::hint::WorkspaceAdjustments;

#[derive(Serialize, Deserialize)]
pub struct ArtieDistance {
    pub family_distance: f64,
    pub block_distance: f64,
    pub position_distance: f64,
    pub input_distance: f64,
    pub total_distance: f64,
    pub workspace_adjustments: WorkspaceAdjustments,
}
impl ArtieDistance {
    pub fn new() -> ArtieDistance {
        ArtieDistance {
            family_distance: 0.0,
            block_distance: 0.0,
            position_distance: 0.0,
            input_distance: 0.0,
            total_distance: 0.0,
            workspace_adjustments: WorkspaceAdjustments::new(),
        }
    }
}