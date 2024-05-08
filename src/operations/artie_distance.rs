use crate::structure::block::Block;
use crate::structure::workspace::Workspace;
use std::collections::HashSet;

pub struct ArtieDistance {
    pub family_distance: f64,
}

/*
    * Artie Distance
    * The Artie distance is a measure of the difference between the workspace and the solution.
 */
pub fn artie_distance(workspace: &Workspace, solution: &Workspace) -> ArtieDistance {
    let mut artie_distance = ArtieDistance {
        family_distance: 0.0,
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
    let _common_families = workspace_families.intersection(&solution_families).count() as f64;
    let unique_families = workspace_families.symmetric_difference(&solution_families).count() as f64;

    // Set the distances for the ArtieDistance struct
    artie_distance.family_distance = unique_families; // Adjust this calculation as needed

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