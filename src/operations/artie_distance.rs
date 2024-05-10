use crate::structure::Block;
use crate::structure::Workspace;
use std::collections::HashSet;

pub struct ArtieDistance {
    pub family_distance: f64,
    pub block_distance: f64,
}

/*
    * Artie Distance
    * The Artie distance is a measure of the difference between the workspace and the solution.
 */
pub fn artie_distance(workspace: &Workspace, solution: &Workspace) -> ArtieDistance {
    let mut artie_distance = ArtieDistance {
        family_distance: 0.0,
        block_distance: 0.0,
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
    let _common_blocks: HashSet<_> = workspace_block_names.intersection(&solution_block_names).cloned().collect();
    let unique_blocks = workspace_block_names.symmetric_difference(&solution_block_names).count() as f64;

    // Set the distances for the ArtieDistance struct
    artie_distance.block_distance += unique_blocks;


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
