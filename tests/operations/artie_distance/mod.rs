use artie_common::structure::Block;
use artie_common::structure::Workspace;
use artie_common::operations::artie_distance::collect_families;
use artie_common::operations::artie_distance::collect_block_names;
use artie_common::operations::artie_distance::artie_distance;
use std::collections::HashSet;
use std::vec;

#[test]
pub fn test_collect_families() {
    
    let mut block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);
    
    next_block.nested.push(nested_next_block);
    block.nested.push(nested_block);
    block.next = Some(Box::new(next_block));

    let mut families = HashSet::new();

    collect_families(&block, &mut families);

    assert_eq!(families.len(), 3);
    
    assert!(families.contains("test_family"));
    assert!(families.contains("test_next_family"));
    assert!(families.contains("test_nested_family"));
    assert_eq!(families.contains("test_nested_next_family"), false);

}

#[test]
pub fn test_collect_block_names(){
    let mut block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);
    
    next_block.nested.push(nested_next_block);
    block.nested.push(nested_block);
    block.next = Some(Box::new(next_block));

    let mut block_names = HashSet::new();

    collect_block_names(&block, &mut block_names);

    assert_eq!(block_names.len(), 4);
    
    assert!(block_names.contains("test_name"));
    assert!(block_names.contains("test_next_name"));
    assert!(block_names.contains("test_nested_name"));
    assert!(block_names.contains("test_nested_next_name"));
}

#[test]
pub fn test_artie_distance_family(){
    
    // Creating the workspace
    let mut workspace_block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut workspace_next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let workspace_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let workspace_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);

    workspace_next_block.nested.push(workspace_nested_next_block);
    workspace_block.nested.push(workspace_nested_block);
    workspace_block.next = Some(Box::new(workspace_next_block));

    let workspace = Workspace::new("workspace_id", "workspace_name", vec![workspace_block]);

    // Creating the solution
    let mut solution_block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut solution_next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let solution_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let solution_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);

    solution_next_block.nested.push(solution_nested_next_block);
    solution_block.nested.push(solution_nested_block);
    solution_block.next = Some(Box::new(solution_next_block));

    let solution = Workspace::new("solution_id", "solution_name", vec![solution_block]);

    //A - Same workspace and solution
    let distance = artie_distance(&workspace, &solution);
    assert_eq!(distance.family_distance, 0.0);

    //B- More elements in workspace than in solution
    let mut workspace_block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut workspace_next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let workspace_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let workspace_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);
    let workspace_extra_block = Block::new("test_extra_id", "test_extra_name", "test_extra_family", vec![]);

    workspace_next_block.nested.push(workspace_nested_next_block);
    workspace_block.nested.push(workspace_nested_block);
    workspace_block.next = Some(Box::new(workspace_next_block));
    workspace_block.next.as_mut().unwrap().nested.push(workspace_extra_block);

    let workspace = Workspace::new("workspace_id", "workspace_name", vec![workspace_block]);

    let distance = artie_distance(&workspace, &solution);
    assert_eq!(distance.family_distance, 1.0);

    //C- More elements in solution than in workspace
    let mut solution_block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut solution_next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let solution_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let solution_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);
    let solution_extra_block = Block::new("test_extra_id", "test_extra_name", "test_extra_family", vec![]);
    let solution_extra_next_block = Block::new("test_extra_next_id_2", "test_extra_next_name_2", "test_extra_family_2", vec![]);

    solution_next_block.nested.push(solution_nested_next_block);
    solution_block.nested.push(solution_nested_block);
    solution_block.next = Some(Box::new(solution_next_block));
    solution_block.next.as_mut().unwrap().nested.push(solution_extra_block);
    solution_block.next.as_mut().unwrap().nested.push(solution_extra_next_block);

    let solution = Workspace::new("solution_id", "solution_name", vec![solution_block]);

    let distance = artie_distance(&workspace, &solution);
    assert_eq!(distance.family_distance, 1.0);

    //D- Completely different workspace and solution
    let mut solution_block = Block::new("test_id", "test_name", "test_family_2", vec![]);
    let mut solution_next_block = Block::new("test_next_id", "test_next_name", "test_next_family_2", vec![]);
    let solution_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family_2", vec![]);
    let solution_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family_2", vec![]);
    let solution_extra_block = Block::new("test_extra_id", "test_extra_name", "test_extra_family_2", vec![]);

    solution_next_block.nested.push(solution_nested_next_block);
    solution_block.nested.push(solution_nested_block);
    solution_block.next = Some(Box::new(solution_next_block));
    solution_block.next.as_mut().unwrap().nested.push(solution_extra_block);

    let solution = Workspace::new("solution_id", "solution_name", vec![solution_block]);
    
    let distance = artie_distance(&workspace, &solution);
    assert_eq!(distance.family_distance, 8.0);

}

#[test]
pub fn test_artie_distance_blocks(){
    
    // Creating the workspace
    let mut workspace_block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut workspace_next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let workspace_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let workspace_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);

    workspace_next_block.nested.push(workspace_nested_next_block);
    workspace_block.nested.push(workspace_nested_block);
    workspace_block.next = Some(Box::new(workspace_next_block));

    let workspace = Workspace::new("workspace_id", "workspace_name", vec![workspace_block]);

    // Creating the solution
    let mut solution_block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut solution_next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let solution_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let solution_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);

    solution_next_block.nested.push(solution_nested_next_block);
    solution_block.nested.push(solution_nested_block);
    solution_block.next = Some(Box::new(solution_next_block));

    let solution = Workspace::new("solution_id", "solution_name", vec![solution_block]);

    //A - Same workspace and solution
    let distance = artie_distance(&workspace, &solution);
    assert_eq!(distance.block_distance, 0.0);

    //B- More elements in workspace than in solution
    let mut workspace_block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut workspace_next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let workspace_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let workspace_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name_new", "test_family", vec![]);
    let workspace_extra_block = Block::new("test_extra_id", "test_extra_name", "test_extra_family", vec![]);

    workspace_next_block.nested.push(workspace_nested_next_block);
    workspace_block.nested.push(workspace_nested_block);
    workspace_block.next = Some(Box::new(workspace_next_block));
    workspace_block.next.as_mut().unwrap().nested.push(workspace_extra_block);

    let workspace = Workspace::new("workspace_id", "workspace_name", vec![workspace_block]);

    let distance = artie_distance(&workspace, &solution);
    assert_eq!(distance.block_distance, 3.0);

    //C- More elements in solution than in workspace
    let mut solution_block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut solution_next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let solution_nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let solution_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);
    let solution = Block::new("test_extra_id", "test_extra_name", "test_extra_family", vec![]);

    solution_next_block.nested.push(solution_nested_next_block);
    solution_block.nested.push(solution_nested_block);
    solution_block.next = Some(Box::new(solution_next_block));
    solution_block.next.as_mut().unwrap().nested.push(solution);

    let solution = Workspace::new("solution_id", "solution_name", vec![solution_block]);

    let distance = artie_distance(&workspace, &solution);
    assert_eq!(distance.block_distance, 2.0);

    //D- Completely different workspace and solution
    let mut solution_block = Block::new("test_id", "test_name_2", "test_family", vec![]);
    let mut solution_next_block = Block::new("test_next_id", "test_next_name_2", "test_next_family", vec![]);
    let solution_nested_block = Block::new("test_nested_id", "test_nested_name_2", "test_nested_family", vec![]);
    let solution_nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name_2", "test_family", vec![]);
    let solution = Block::new("test_extra_id", "test_extra_name_2", "test_extra_family", vec![]);
    
    solution_next_block.nested.push(solution_nested_next_block);
    solution_block.nested.push(solution_nested_block);
    solution_block.next = Some(Box::new(solution_next_block));
    solution_block.next.as_mut().unwrap().nested.push(solution);

    let solution = Workspace::new("solution_id", "solution_name", vec![solution_block]);

    let distance = artie_distance(&workspace, &solution);
    assert_eq!(distance.block_distance, 10.0);

}

