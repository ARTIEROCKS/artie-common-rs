use artie_common::structure::Block;
use artie_common::structure::Field;
use artie_common::structure::Input;
use artie_common::structure::Workspace;
use std::vec;
use artie_common::structure::ArtieDistance;
use artie_common::operations::artie_distance::calculate_input_distance;


#[test]
pub fn test_calculate_input_distance_same() {

        /*
        Creates solution_block_a with two fields: 
        one called solution_input_a with name solution_input_a with a field with name input_a_value and random string value
        another called solution_input_b with name solution_input_b with a field with name input_b_value and random double value.
        Then we create the solution workspace
        */
        let mut solution_block_a = Block::new("test_id", "test_name", "test_family", vec![]);
        let input_a = Input::new("input_a", "input_a_value", vec![Field::new(&"input_a_value".to_string(), &"input_a_value".to_string())]);
        solution_block_a.add_input(input_a);

        let solution = Workspace::new("solution", "solution", vec![solution_block_a]);

        /* Does the same wit the workspace but renaming the blocks, fields and workspace */
        let mut workspace_block_a = Block::new("test_id", "test_name", "test_family", vec![]);
        let input_a = Input::new("input_a", "input_a_value", vec![Field::new(&"input_a_value".to_string(), &"input_a_value".to_string())]);
        workspace_block_a.add_input(input_a);

        let workspace = Workspace::new("workspace", "workspace", vec![workspace_block_a]);

        // Tests the calculate_input_distance function
        let mut artie_distance = ArtieDistance::new();
        calculate_input_distance(&workspace.blocks[0], &solution.blocks[0], &mut artie_distance);

        assert_eq!(artie_distance.input_distance, 0.0);

        // Checks the workspace adjustments
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_add.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_remove.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_reposition.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_with_input_changes.len(), 0);

}

#[test]
pub fn test_calculate_input_distance_difference_in_string_value(){
    
       /*
        Creates solution_block_a with two fields: 
        one called solution_input_a with name solution_input_a with a field with name input_a_value and random string value
        another called solution_input_b with name solution_input_b with a field with name input_b_value and random double value.
        Then we create the solution workspace
        */
        let mut solution_block_a = Block::new("test_id", "test_name", "test_family", vec![]);
        let input_a = Input::new("input_a", "input_a_value", vec![Field::new(&"input_a_value".to_string(), &"input_a_value".to_string())]);
        solution_block_a.add_input(input_a);
    
        let solution = Workspace::new("solution", "solution", vec![solution_block_a]);
    
        /* Does the same wit the workspace but renaming the blocks, fields and workspace */
        let mut workspace_block_a = Block::new("test_id", "test_name", "test_family", vec![]);
        let input_a = Input::new("input_a", "input_a_value", vec![Field::new(&"input_a_value".to_string(), &"workspace_input_a_value".to_string())]);
        workspace_block_a.add_input(input_a);

    
        let workspace = Workspace::new("workspace", "workspace", vec![workspace_block_a]);
    
        // Tests the calculate_input_distance function
        let mut artie_distance = ArtieDistance::new();
        calculate_input_distance(&workspace.blocks[0], &solution.blocks[0], &mut artie_distance);
    
        assert_eq!(artie_distance.input_distance, 1.0);

        // Checks the workspace adjustments
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_add.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_remove.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_reposition.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_with_input_changes.len(), 1);

}

#[test]
pub fn test_calculate_input_distance_difference_in_double_value(){
    
        /*
        Creates solution_block_a with two fields: 
        one called solution_input_a with name solution_input_a with a field with name input_a_value and random string value
        another called solution_input_b with name solution_input_b with a field with name input_b_value and random double value.
        Then we create the solution workspace
        */
        let mut solution_block_a = Block::new("test_id", "test_name", "test_family", vec![]);
        let input_a = Input::new("input_a", "input_a_value", vec![Field::new(&"input_a_value".to_string(), &"8.45".to_string())]);
        solution_block_a.add_input(input_a);
    
        let solution = Workspace::new("solution", "solution", vec![solution_block_a]);
    
        /* Does the same wit the workspace but renaming the blocks, fields and workspace */
        let mut workspace_block_a = Block::new("test_id", "test_name", "test_family", vec![]);
        let input_a = Input::new("input_a", "input_a_value", vec![Field::new(&"input_a_value".to_string(), &"8.35".to_string())]);
        workspace_block_a.add_input(input_a);
    
        let workspace = Workspace::new("workspace", "workspace", vec![workspace_block_a]);
    
        // Tests the calculate_input_distance function
        let mut artie_distance = ArtieDistance::new();
        calculate_input_distance(&workspace.blocks[0], &solution.blocks[0], &mut artie_distance);
    
        let tolerance = 1e-10;
        assert!((artie_distance.input_distance - 0.1).abs() < tolerance, "artie_distance.input_distance = {}", artie_distance.input_distance);

        // Checks the workspace adjustments
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_add.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_remove.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_reposition.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_with_input_changes.len(), 1);

}

#[test]
pub fn test_calculate_input_distance_differences_in_all_inputs(){
        
        /*
        Creates solution_block_a with two fields: 
        one called solution_input_a with name solution_input_a with a field with name input_a_value and random string value
        another called solution_input_b with name solution_input_b with a field with name input_b_value and random double value.
        Then we create the solution workspace
        */
        let mut solution_block_a = Block::new("test_id", "test_name", "test_family", vec![]);
        let input_a = Input::new("input_a", "input_a_value", vec![Field::new(&"input_a_value".to_string(), &"input_a_value".to_string())]);
        let input_b = Input::new("input_b", "input_b_value", vec![Field::new(&"input_b_value".to_string(), &"3.3".to_string())]);
        solution_block_a.add_input(input_a);
        solution_block_a.add_input(input_b);

        let solution = Workspace::new("solution", "solution", vec![solution_block_a]);

        /* Does the same wit the workspace but renaming the blocks, fields and workspace */
        let mut workspace_block_a = Block::new("test_id", "test_name", "test_family", vec![]);
        let input_a = Input::new("input_a", "input_a_value", vec![Field::new(&"input_a_value".to_string(), &"workspace_input_a_value".to_string())]);
        let input_b = Input::new("input_b", "input_b_value", vec![Field::new(&"input_b_value".to_string(), &"8.35".to_string())]);
        workspace_block_a.add_input(input_a);
        workspace_block_a.add_input(input_b);

        let workspace = Workspace::new("workspace", "workspace", vec![workspace_block_a]);

        // Tests the calculate_input_distance function
        let mut artie_distance = ArtieDistance::new();
        calculate_input_distance(&workspace.blocks[0], &solution.blocks[0], &mut artie_distance);

        let tolerance = 1e-10;
        assert!((artie_distance.input_distance - 6.05).abs() < tolerance, "artie_distance.input_distance = {}", artie_distance.input_distance);

        // Checks the workspace adjustments
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_add.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_remove.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_to_reposition.len(), 0);
        assert_eq!(artie_distance.workspace_adjustments.blocks_with_input_changes.len(), 2);
}