use artie_common::structure::Workspace;
use artie_common::structure::Block;

#[test]
pub fn test_add_block() {
    let block_1 = Block::new("test", "test", "test", vec![]);
    let block_2 = Block::new("test", "test", "test", vec![]);
    let mut workspace = Workspace::new("test", "test", vec![block_1]);
    workspace.add_block(block_2);
    assert_eq!(workspace.blocks.len(), 2);
}

#[test]
pub fn test_workspace_eq() {
    let block_1 = Block::new("test", "test", "test", vec![]);
    let block_2 = Block::new("test", "test", "test", vec![]);
    let block_3 = Block::new("test", "test", "test", vec![]);
    let mut workspace_1 = Workspace::new("test", "test", vec![block_1]);
    let workspace_2 = Workspace::new("test", "test", vec![block_2]);
    assert_eq!(workspace_1, workspace_2);
    workspace_1.add_block(block_3);
    assert_ne!(workspace_1, workspace_2);
}        
