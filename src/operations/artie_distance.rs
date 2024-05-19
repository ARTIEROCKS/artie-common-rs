use crate::structure::hint::BlockPositionChange;
use crate::structure::Block;
use crate::structure::Workspace;
use crate::structure::hint::WorkspaceAdjustments;
use crate::structure::hint::BlockChange;
use std::collections::HashSet;

// Coefficients for weighting each distance type
const FAMILY_DISTANCE_COEFFICIENT: f64 = 1.0;
const BLOCK_DISTANCE_COEFFICIENT: f64 = 2.0;
const POSITION_DISTANCE_COEFFICIENT: f64 = 4.0;
const INPUT_DISTANCE_COEFFICIENT: f64 = 8.0;

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

/*
    * Artie Distance
    * The Artie distance is a measure of the difference between the workspace and the solution.
 */
pub fn artie_distance(workspace: &Workspace, solution: &Workspace) -> ArtieDistance {
    let mut artie_distance = ArtieDistance::new();

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

    // Unique blocks in workspace (to be removed)
    let unique_workspace_blocks = workspace_block_names.difference(&solution_block_names).cloned().collect::<HashSet<_>>();

    // Unique blocks in solution (to be added)
    let unique_solution_blocks = solution_block_names.difference(&workspace_block_names).cloned().collect::<HashSet<_>>();

    // Set the distances for the ArtieDistance struct
    artie_distance.block_distance += unique_workspace_blocks.len() as f64;
    artie_distance.block_distance += unique_solution_blocks.len() as f64;


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

    // Add unique blocks from the workspace to blocks_to_remove
    workspace.blocks.iter()
        .filter(|block| unique_workspace_blocks.contains(&block.name))
        .for_each(|block| {
            artie_distance.workspace_adjustments.blocks_to_remove.push(BlockChange {
                id: block.id.clone(),
                name: block.name.clone(),
            });
        });

    // Add unique blocks from the solution to blocks_to_add
    solution.blocks.iter()
        .filter(|block| unique_solution_blocks.contains(&block.name))
        .for_each(|block| {
            artie_distance.workspace_adjustments.blocks_to_add.push(BlockChange {
                id: block.id.clone(),
                name: block.name.clone(),
            });
        });

    // 3.2- Calculate the position distance for unique_blocks
    for block in &workspace_block_positions {
        if unique_workspace_blocks.contains(&block.0) {
            artie_distance.position_distance += (block.1 as i32).abs() as f64;
        }
    }
    for block in &solution_block_positions {
        if unique_solution_blocks.contains(&block.0) {
            artie_distance.position_distance += (block.1 as i32).abs() as f64;
        }
    }

    // 3.3- Calculate the position distance for common_blocks (blocks that are in the workspace and solution)
    let mut solution_positions_remaining = solution_block_positions.clone();
    for workspace_block in &workspace_block_positions {

        if common_blocks.contains(&workspace_block.0) {
            
            // Checks if the block exists in the solution positions remaining, and gets the index and the element
            if let Some((index, solution_block)) = solution_positions_remaining.iter().enumerate().find(|(_, block)| block.0 == workspace_block.0) {
                // Calculate the position distance
                let distance = (workspace_block.1 as i32 - solution_block.1 as i32).abs() as f64;
                artie_distance.position_distance += distance;

                if distance > 0.0 {
                    // Adds the workspace block to reposition
                    artie_distance.workspace_adjustments.blocks_to_reposition.push(BlockPositionChange {
                        block: BlockChange {
                                id: workspace_block.0.clone(),
                                name : workspace_block.0.clone(),
                            },
                        current_position: vec![workspace_block.1],
                        target_position: vec![solution_block.1],
                        }
                    );
                }

                // Remove the element from the solution positions remaining
                solution_positions_remaining.swap_remove(index);
            }else{
                artie_distance.position_distance += workspace_block.1 as f64;
            }
            
        }
    }

    // 4- Input distance calculation
    for workspace_block in &workspace.blocks {
        for solution_block in &solution.blocks {
            calculate_input_distance(workspace_block, solution_block, &mut artie_distance);
        }
    }

    // 5- Total distance calculation
    calculate_total_distance(&mut artie_distance);

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

    if workspace_block.name == solution_block.name {
        for workspace_field in &workspace_block.fields {
            if let Some(solution_field) = solution_block.fields.iter().find(|f| f.name == workspace_field.name) {
                if workspace_field.value != solution_field.value && workspace_field.is_numeric() && solution_field.is_numeric() {
                    artie_distance.input_distance += (workspace_field.value_as_double() - solution_field.value_as_double()).abs();
                } else if workspace_field.value != solution_field.value {
                    artie_distance.input_distance += 1.0;
                }
            }
        }
    }

    // Recursively calculate the input distance for nested blocks
    for workspace_nested_block in &workspace_block.nested {
        if let Some(solution_nested_block) = solution_block.nested.iter().find(|b| b.name == workspace_nested_block.name) {
            calculate_input_distance(workspace_nested_block, solution_nested_block, artie_distance);
        }
    }

    // Recursively calculate the input distance for the next block
    if let (Some(workspace_next_block), Some(solution_next_block)) = (&workspace_block.next, &solution_block.next) {
        calculate_input_distance(workspace_next_block, solution_next_block, artie_distance);
    }
}

pub fn calculate_total_distance(artie_distance: &mut ArtieDistance) {
    artie_distance.total_distance = 
          (artie_distance.family_distance / FAMILY_DISTANCE_COEFFICIENT)
        + (artie_distance.block_distance / BLOCK_DISTANCE_COEFFICIENT)
        + (artie_distance.position_distance / POSITION_DISTANCE_COEFFICIENT)
        + (artie_distance.input_distance / INPUT_DISTANCE_COEFFICIENT);
}