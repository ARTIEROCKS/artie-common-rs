use crate::structure::Block;
use crate::structure::Workspace;
use std::collections::HashSet;

pub struct ArtieDistance {
    pub family_distance: f64,
    pub block_distance: f64,
    pub position_distance: f64,
    pub input_distance: f64,
}

/*
    * Artie Distance
    * The Artie distance is a measure of the difference between the workspace and the solution.
 */
pub fn artie_distance(workspace: &Workspace, solution: &Workspace) -> ArtieDistance {
    let mut artie_distance = ArtieDistance {
        family_distance: 0.0,
        block_distance: 0.0,
        position_distance: 0.0,
        input_distance: 0.0,
    };

    //1- Family distance calculation
    let mut workspace_families = HashSet::new();
    let mut solution_families = HashSet::new();

    // Recursively obtain the families from the workspace and solution
    for block in &workspace.blocks {
        collect_families(block, &mut workspace_families);
    }
    for block in &solution.blocks {
        collect_families(block, &mut solution_families);
    }

    // Calculate the family distance
    let unique_families = workspace_families.symmetric_difference(&solution_families).count() as f64;

    // Set the distances for the ArtieDistance struct
    artie_distance.family_distance = unique_families;



    //2- Block distance calculation
    let mut workspace_block_names = HashSet::new();
    let mut solution_block_names = HashSet::new();

    // Recursively obtain the block names from the workspace and solution
    for block in &workspace.blocks {
        collect_block_names(block, &mut workspace_block_names);
    }
    for block in &solution.blocks {
        collect_block_names(block, &mut solution_block_names);
    }

    // Calculate the block distance
    let common_blocks: HashSet<_> = workspace_block_names.intersection(&solution_block_names).cloned().collect();
    let unique_blocks: HashSet<_> = workspace_block_names.symmetric_difference(&solution_block_names).cloned().collect();

    // Set the distances for the ArtieDistance struct
    artie_distance.block_distance += unique_blocks.len() as f64;


    // 3- Position distance calculation
    // 3.1- Collect the block positions for the workspace and solution
    let mut workspace_block_positions = Vec::new();
    let mut solution_block_positions = Vec::new();

    let mut position = 1;
    for block in &workspace.blocks {
        collect_block_positions(block, &mut position, &mut workspace_block_positions);
    }

    position = 1;
    for block in &solution.blocks {
        collect_block_positions(block, &mut position, &mut solution_block_positions);
    }

    // 3.2- Calculate the position distance for unique_blocks
    for block in &workspace_block_positions {
        if unique_blocks.contains(&block.0) {
            artie_distance.position_distance += (block.1 as i32).abs() as f64;
        }
    }
    for block in &solution_block_positions {
        if unique_blocks.contains(&block.0) {
            artie_distance.position_distance += (block.1 as i32).abs() as f64;
        }
    }

    // 3.3- Calculate the position distance for common_blocks (blocks that are in the workspace and solution)
    for block in workspace_block_positions {
        if common_blocks.contains(&block.0) {
            let solution_position = solution_block_positions.iter().find(|&x| x.0 == block.0).unwrap();
            artie_distance.position_distance += (block.1 as i32 - solution_position.1 as i32).abs() as f64;
        }
    }

    // 4- Input distance calculation
    for workspace_block in &workspace.blocks {
        for solution_block in &solution.blocks {
            calculate_input_distance(workspace_block, solution_block, &mut artie_distance);
        }
    }

    artie_distance

}

// Helper function to recursively collect families from blocks
pub fn collect_families(block: &Block, families: &mut HashSet<String>) {
    families.insert(block.family.clone());
    for nested_block in &block.nested {
        collect_families(nested_block, families);
    }
    if let Some(next_block) = &block.next {
        collect_families(&next_block, families);
    }
}

// Helper function ro recursively collect block names from blocks that exists in the common families hashset
pub fn collect_block_names(block: &Block, block_names: &mut HashSet<String>) {
    block_names.insert(block.name.clone());
    for nested_block in &block.nested {
        collect_block_names(nested_block, block_names);
    }
    if let Some(next_block) = &block.next {
        collect_block_names(&next_block, block_names);
    }
}

// Helper to calculate the position for each block
pub fn collect_block_positions(block: &Block, position: &mut usize, block_positions: &mut Vec<(String, usize)>){

    // Insert the block name and its position in the block_positions vector
    block_positions.push((block.name.clone(), *position));
    
    // Adds 1 to the position for the next block
    *position += 1;

    // Recursively call the function for the nested block
    for nested_block in &block.nested {
        // Adds 1 extra value to the nested position
        *position += 1;
        collect_block_positions(nested_block, position, block_positions);
    }

    // Recursively call the function for the next block
    if let Some(next_block) = &block.next {
        collect_block_positions(&next_block, position, block_positions);
    }
}

// Helper function to calculate the input distance
pub fn calculate_input_distance(workspace_block: &Block, solution_block: &Block, artie_distance: &mut ArtieDistance) {
   
   // Calculating the input distance
   if workspace_block.name == solution_block.name {
        for (workspace_field, solution_field) in workspace_block.fields.iter().zip(solution_block.fields.iter()) {
            if workspace_field.name == solution_field.name {
                if workspace_field.value != solution_field.value && workspace_field.is_numeric(){
                    artie_distance.input_distance += (workspace_field.value_as_double() - solution_field.value_as_double()).abs();
                }else if workspace_field.value != solution_field.value {
                    artie_distance.input_distance += 1.0;
                }
            }
        }
    }

    // Doing the calculations for nested blocks
    for (workspace_nested_block, solution_nested_block) in workspace_block.nested.iter().zip(solution_block.nested.iter()) {
        calculate_input_distance(workspace_nested_block, solution_nested_block, artie_distance);
    }

    // Doing the calculations for the next block
    if let Some(workspace_next_block) = &workspace_block.next {
        if let Some(solution_next_block) = &solution_block.next {
            calculate_input_distance(workspace_next_block, solution_next_block, artie_distance);
        }
    }
}