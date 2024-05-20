use artie_common::structure::Block;
use artie_common::structure::Workspace;
use artie_common::operations::artie_distance::collect_block_positions;
use artie_common::operations::artie_distance::artie_distance;
use std::vec;

#[test]
fn test_collect_block_positions(){
    let mut block = Block::new("test_id", "test_name", "test_family", vec![]);
    let mut next_block = Block::new("test_next_id", "test_next_name", "test_next_family", vec![]);
    let nested_block = Block::new("test_nested_id", "test_nested_name", "test_nested_family", vec![]);
    let nested_next_block = Block::new("test_nested_next_id", "test_nested_next_name", "test_family", vec![]);
    
    next_block.nested.push(nested_next_block);
    block.nested.push(nested_block);
    block.next = Some(Box::new(next_block));

    let mut block_positions = Vec::new();
    let mut position = 0;
    collect_block_positions(&block, &mut position, &mut block_positions);

    assert_eq!(block_positions.len(), 4);
    assert_eq!(block_positions[0].0, "test_name");
    assert_eq!(block_positions[0].1, 0);
    assert_eq!(block_positions[1].0, "test_nested_name");
    assert_eq!(block_positions[1].1, 2);
    assert_eq!(block_positions[2].0, "test_next_name");
    assert_eq!(block_positions[2].1, 3);
    assert_eq!(block_positions[3].0, "test_nested_next_name");
    assert_eq!(block_positions[3].1, 5);
}

#[test]
pub fn test_artie_distance_positions_same(){
    
    // Creating the solution
    let mut solution_block_a = Block::new("A", "A", "test_family", vec![]);
    let mut solution_block_b = Block::new("B", "B", "test_family", vec![]);
    let mut solution_block_c = Block::new("C", "C", "test_family", vec![]);
    let solution_block_d = Block::new("D", "D", "test_family", vec![]);

    // D is nested in C
    solution_block_c.nested.push(solution_block_d);
    // C is next to B
    solution_block_b.next = Some(Box::new(solution_block_c));
    // B is nested in A
    solution_block_a.nested.push(solution_block_b);

    // Creating the workspace
    let mut workspace_block_a = Block::new("A", "A", "test_family", vec![]);
    let mut workspace_block_b = Block::new("B", "B", "test_family", vec![]);
    let mut workspace_block_c = Block::new("C", "C", "test_family", vec![]);
    let workspace_block_d = Block::new("D", "D", "test_family", vec![]);

    // D is nested in C
    workspace_block_c.nested.push(workspace_block_d);
    // C is next to B
    workspace_block_b.next = Some(Box::new(workspace_block_c));
    // B is nested in A
    workspace_block_a.nested.push(workspace_block_b);

    let distance = artie_distance(&Workspace::new("workspace_id", "workspace_name", vec![workspace_block_a]), &Workspace::new("solution_id", "solution_name", vec![solution_block_a]));
    assert_eq!(distance.position_distance, 0.0);

    // Checks the workspace adjustments
    assert_eq!(distance.workspace_adjustments.blocks_to_add.len(), 0);
    assert_eq!(distance.workspace_adjustments.blocks_to_remove.len(), 0);
    assert_eq!(distance.workspace_adjustments.blocks_to_reposition.len(), 0);
    assert_eq!(distance.workspace_adjustments.blocks_with_input_changes.len(), 0);

}

#[test]
pub fn test_artie_distance_positions_more_in_workspace(){

    // Creating the solution
    let mut solution_block_a = Block::new("A", "A", "test_family", vec![]);
    let mut solution_block_b = Block::new("B", "B", "test_family", vec![]);
    let mut solution_block_c = Block::new("C", "C", "test_family", vec![]);
    let solution_block_d = Block::new("D", "D", "test_family", vec![]);

    // D is nested in C
    solution_block_c.nested.push(solution_block_d);
    // C is next to B
    solution_block_b.next = Some(Box::new(solution_block_c));
    // B is nested in A
    solution_block_a.nested.push(solution_block_b);


    // Creating the workspace
    let mut workspace_block_a = Block::new("A", "A", "test_family", vec![]);
    let mut workspace_block_b = Block::new("B", "B", "test_family", vec![]);
    let mut workspace_block_c = Block::new("C", "C", "test_family", vec![]);
    let workspace_block_d = Block::new("D", "D", "test_family", vec![]);
    let workspace_block_e = Block::new("E", "E", "test_family", vec![]);

    // D is nested in C
    workspace_block_c.nested.push(workspace_block_d);
    // C is next to B
    workspace_block_b.next = Some(Box::new(workspace_block_c));
    // B is nested in A
    workspace_block_a.nested.push(workspace_block_b);
    //E is next to A
    workspace_block_a.next = Some(Box::new(workspace_block_e));

    // Asserting that the distance is 7
    let distance = artie_distance(&Workspace::new("workspace_id", "workspace_name", vec![workspace_block_a]), &Workspace::new("solution_id", "solution_name", vec![solution_block_a]));
    assert_eq!(distance.position_distance, 7.0);

    // Checks the workspace adjustments
    assert_eq!(distance.workspace_adjustments.blocks_to_add.len(), 0);
    assert_eq!(distance.workspace_adjustments.blocks_to_remove.len(), 1);
    assert_eq!(distance.workspace_adjustments.blocks_to_reposition.len(), 0);
    assert_eq!(distance.workspace_adjustments.blocks_with_input_changes.len(), 0);

}

#[test]
pub fn test_artie_distance_positions_more_in_solution(){
    // Creating the solution
    let mut solution_block_a = Block::new("A", "A", "test_family", vec![]);
    let mut solution_block_b = Block::new("B", "B", "test_family", vec![]);
    let solution_block_c = Block::new("C", "C", "test_family", vec![]);
    let solution_block_d = Block::new("D", "D", "test_family", vec![]);
    let solution_block_e = Block::new("E", "E", "test_family", vec![]);

    // C is nested in B
    solution_block_b.nested.push(solution_block_c);
    // D is next to B
    solution_block_b.next = Some(Box::new(solution_block_d));
    // B is nested in A
    solution_block_a.nested.push(solution_block_b);
    // E is next to A
    solution_block_a.next = Some(Box::new(solution_block_e));


    // Creating the workspace
    let mut workspace_block_a = Block::new("A", "A", "test_family", vec![]);
    let mut workspace_block_b = Block::new("B", "B", "test_family", vec![]);
    let mut workspace_block_c = Block::new("C", "C", "test_family", vec![]);
    let workspace_block_d = Block::new("D", "D", "test_family", vec![]);
 
    // D is nested in C
    workspace_block_c.nested.push(workspace_block_d);
    // C is next to B
    workspace_block_b.next = Some(Box::new(workspace_block_c));
    // B is nested in A
    workspace_block_a.nested.push(workspace_block_b);
 
    // Asserting that the distance is 8
    let distance = artie_distance(&Workspace::new("workspace_id", "workspace_name", vec![workspace_block_a]), &Workspace::new("solution_id", "solution_name", vec![solution_block_a]));
    assert_eq!(distance.position_distance, 8.0);

    // Checks the workspace adjustments
    assert_eq!(distance.workspace_adjustments.blocks_to_add.len(), 1);
    assert_eq!(distance.workspace_adjustments.blocks_to_remove.len(), 0);
    assert_eq!(distance.workspace_adjustments.blocks_to_reposition.len(), 1);
    assert_eq!(distance.workspace_adjustments.blocks_with_input_changes.len(), 0);

}

#[test]
pub fn test_artie_distance_positions_completely_different(){
    
    // Creating the solution
    let mut solution_block_a = Block::new("A", "A", "test_family", vec![]);
    let mut solution_block_b = Block::new("B", "B", "test_family", vec![]);
    let solution_block_c = Block::new("C", "C", "test_family", vec![]);

    // C is next to B
    solution_block_b.next = Some(Box::new(solution_block_c));
    // B is nested in A
    solution_block_a.nested.push(solution_block_b);


    // Creating the workspace
    let mut workspace_block_d = Block::new("D", "D", "test_family", vec![]);
    let mut workspace_block_e = Block::new("E", "E", "test_family", vec![]);
    let workspace_block_f = Block::new("F", "F", "test_family", vec![]);
    let workspace_block_g = Block::new("G", "G", "test_family", vec![]);

    // F is next to E
    workspace_block_e.next = Some(Box::new(workspace_block_f));
    // E is nested in D
    workspace_block_d.nested.push(workspace_block_e);
    // G is next to D
    workspace_block_d.next = Some(Box::new(workspace_block_g));

    // Asserting that the distance is 21
    let distance = artie_distance(&Workspace::new("workspace_id", "workspace_name", vec![workspace_block_d]), &Workspace::new("solution_id", "solution_name", vec![solution_block_a]));
    assert_eq!(distance.position_distance, 21.0);

    // Checks the workspace adjustments
    assert_eq!(distance.workspace_adjustments.blocks_to_add.len(), 3);
    assert_eq!(distance.workspace_adjustments.blocks_to_remove.len(), 4);
    assert_eq!(distance.workspace_adjustments.blocks_to_reposition.len(), 0);
    assert_eq!(distance.workspace_adjustments.blocks_with_input_changes.len(), 0);

}
