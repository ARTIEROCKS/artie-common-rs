// Structure to represent a block that needs to be added or removed, with the necessary information.
#[derive(Debug, Clone)]
pub struct BlockChange {
    pub id: String,
    pub name: String,
}

// Structure to represent a position change of a block.
#[derive(Debug, Clone)]
pub struct BlockPositionChange {
    pub block: BlockChange,
    pub current_position: Vec<usize>, // Current position in the student's workspace.
    pub target_position: Vec<usize>,  // Target correct position.
}

// Structure to represent an incorrect input in a block.
#[derive(Debug, Clone)]
pub struct InputChange {
    pub block_id: String,       // ID of the block containing the incorrect input.
    pub input_name: String,     // Name of the input.
    pub expected_value: String, // Expected value for that input.
    pub actual_value: String,   // Actual value of the input in the student's workspace.
}

// Main structure to store all necessary adjustments.
#[derive(Debug)]
pub struct WorkspaceAdjustments {
    pub blocks_to_remove: Vec<BlockChange>,
    pub blocks_to_add: Vec<BlockChange>,
    pub blocks_to_reposition: Vec<BlockPositionChange>,
    pub blocks_with_input_changes: Vec<InputChange>,
}

impl WorkspaceAdjustments {
    // Associated function to create a new instance of WorkspaceAdjustments.
    pub fn new() -> Self {
        WorkspaceAdjustments {
            blocks_to_remove: Vec::new(),
            blocks_to_add: Vec::new(),
            blocks_to_reposition: Vec::new(),
            blocks_with_input_changes: Vec::new(),
        }
    }
}