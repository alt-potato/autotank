use crate::physics::collision::AABB;
use crate::util::math::{ConvertToScalar, Scalar};
use std::collections::HashSet;

/// A spatial hashmap for storing objects (with AABB bounding boxes) in a 2D grid.
///
/// Uses a grid of cells, where each cell contains the set of objects in that cell.
pub struct SpatialHashMap {
    map_width: Scalar,
    map_height: Scalar,
    cell_width: Scalar,
    inv_cell_width: Scalar,
    cell_height: Scalar,
    inv_cell_height: Scalar,
    grid_width: u32,  // width in cells
    grid_height: u32, // height in cells
    grid: Vec<HashSet<u32>>,
}

impl SpatialHashMap {
    pub fn new(map_width: Scalar, map_height: Scalar, grid_width: u32, grid_height: u32) -> Self {
        let cell_width = map_width / grid_width.to_scalar();
        let cell_height = map_height / grid_height.to_scalar();

        SpatialHashMap {
            map_width,
            map_height,
            cell_width,
            inv_cell_width: 1.0.to_scalar() / cell_width,
            cell_height,
            inv_cell_height: 1.0.to_scalar() / cell_height,
            grid_width,
            grid_height,
            grid: vec![HashSet::new(); (grid_width * grid_height) as usize],
        }
    }

    /// Returns the keys of all the cells that contain the given AABB.
    pub fn keys_iter(&self, aabb: &AABB) -> impl Iterator<Item = u32> + use<> {
        // clamp AABB to be within the map bounds
        let min_x = aabb.min.x.clamp(0.0.to_scalar(), self.map_width);
        let min_y = aabb.min.y.clamp(0.0.to_scalar(), self.map_height);
        let max_x = aabb.max.x.clamp(0.0.to_scalar(), self.map_width);
        let max_y = aabb.max.y.clamp(0.0.to_scalar(), self.map_height);

        // convert AABB to cell coordinates
        let min_x_idx = (min_x * self.inv_cell_width)
            .floor()
            .to_u32()
            .unwrap_or(0)
            .clamp(0, self.grid_width - 1);
        let min_y_idx = (min_y * self.inv_cell_height)
            .floor()
            .to_u32()
            .unwrap_or(0)
            .clamp(0, self.grid_height - 1);
        let max_x_idx = (max_x * self.inv_cell_width)
            .floor()
            .to_u32()
            .unwrap_or(0)
            .clamp(0, self.grid_width - 1);
        let max_y_idx = (max_y * self.inv_cell_height)
            .floor()
            .to_u32()
            .unwrap_or(0)
            .clamp(0, self.grid_height - 1);

        // prevent iterator from capturing self =s
        let grid_width = self.grid_width;

        // create iterator over cell keys
        (min_y_idx..=max_y_idx)
            .flat_map(move |y| (min_x_idx..=max_x_idx).map(move |x| x + y * grid_width))
    }

    /// Inserts an object with the given AABB into the grid.
    pub fn insert(&mut self, object_id: u32, aabb: &AABB) {
        for key in self.keys_iter(aabb) {
            if let Some(cell) = self.grid.get_mut(key as usize) {
                cell.insert(object_id);
            }
        }
    }

    /// Returns all unique object IDs in the specified cell.
    pub fn get(&self, key: u32) -> HashSet<u32> {
        self.grid.get(key as usize).cloned().unwrap_or_default()
    }

    /// Returns all unique object IDs that overlap with the given AABB.
    pub fn query(&self, aabb: &AABB) -> HashSet<u32> {
        let mut result = HashSet::new();

        for key in self.keys_iter(aabb) {
            if let Some(cell) = self.grid.get(key as usize) {
                result.extend(cell);
            }
        }

        result
    }

    /// Clears all objects from the grid.
    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            cell.clear();
        }
    }
}

// ...existing code...

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::math::{ConvertToScalar, Vec2};
    use std::collections::HashSet;

    // Helper to create an AABB
    fn create_aabb(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> AABB {
        AABB::new(
            Vec2::new(min_x.to_scalar(), min_y.to_scalar()),
            Vec2::new(max_x.to_scalar(), max_y.to_scalar()),
        )
    }

    #[test]
    fn spatial_hashmap_should_have_basic_functionality() {
        let map_width = 100.0.to_scalar();
        let map_height = 100.0.to_scalar();
        let grid_width = 10;
        let grid_height = 10;
        let mut shm = SpatialHashMap::new(map_width, map_height, grid_width, grid_height);

        // Object 1: (5,5) - (15,15), fits in one cell (0,0)
        let aabb1 = create_aabb(5.0, 5.0, 15.0, 15.0);
        let obj_id1 = 1;
        shm.insert(obj_id1, &aabb1);

        // Object 2: (80,80) - (90,90), fits in one cell (8,8)
        let aabb2 = create_aabb(80.0, 80.0, 90.0, 90.0);
        let obj_id2 = 2;
        shm.insert(obj_id2, &aabb2);

        // Query for object 1
        let query_aabb1 = create_aabb(10.0, 10.0, 10.0, 10.0); // Point query in object 1's cell
        let results1 = shm.query(&query_aabb1);
        assert!(results1.contains(&obj_id1));
        assert!(!results1.contains(&obj_id2));
        assert_eq!(results1.len(), 1);

        // Query for object 2
        let query_aabb2 = create_aabb(85.0, 85.0, 85.0, 85.0); // Point query in object 2's cell
        let results2 = shm.query(&query_aabb2);
        assert!(results2.contains(&obj_id2));
        assert!(!results2.contains(&obj_id1));
        assert_eq!(results2.len(), 1);

        // Query an empty area
        let query_aabb_empty = create_aabb(40.0, 40.0, 45.0, 45.0);
        let results_empty = shm.query(&query_aabb_empty);
        assert!(results_empty.is_empty());

        // Clear the map
        shm.clear();
        let results_after_clear = shm.query(&query_aabb1);
        assert!(results_after_clear.is_empty());
    }

    #[test]
    fn spatial_hashmap_when_object_in_single_cell_should_be_found_in_that_cell() {
        let mut shm = SpatialHashMap::new(20.0.to_scalar(), 20.0.to_scalar(), 2, 2); // 4 cells, each 10x10
        let obj_id = 100;
        // AABB (1,1)-(9,9) should be in cell (0,0)
        let aabb = create_aabb(1.0, 1.0, 9.0, 9.0);
        shm.insert(obj_id, &aabb);

        let query_cell00 = create_aabb(5.0, 5.0, 5.0, 5.0); // Query point in cell (0,0)
        let results00 = shm.query(&query_cell00);
        assert!(results00.contains(&obj_id));
        assert_eq!(results00.len(), 1);

        let query_cell01 = create_aabb(15.0, 5.0, 15.0, 5.0); // Query point in cell (1,0)
        let results01 = shm.query(&query_cell01);
        assert!(results01.is_empty());
    }

    #[test]
    fn spatial_hashmap_when_object_spans_multiple_cells_should_be_found_in_all_cells() {
        let mut shm = SpatialHashMap::new(20.0.to_scalar(), 20.0.to_scalar(), 2, 2); // 4 cells, each 10x10
        let obj_id = 200;
        // AABB (5,5)-(15,15) spans cells (0,0), (1,0), (0,1), (1,1)
        let aabb = create_aabb(5.0, 5.0, 15.0, 15.0);
        shm.insert(obj_id, &aabb);

        let query_cell00 = create_aabb(6.0, 6.0, 6.0, 6.0);
        assert!(shm.query(&query_cell00).contains(&obj_id));
        let query_cell10 = create_aabb(14.0, 6.0, 14.0, 6.0);
        assert!(shm.query(&query_cell10).contains(&obj_id));
        let query_cell01 = create_aabb(6.0, 14.0, 6.0, 14.0);
        assert!(shm.query(&query_cell01).contains(&obj_id));
        let query_cell11 = create_aabb(14.0, 14.0, 14.0, 14.0);
        assert!(shm.query(&query_cell11).contains(&obj_id));
    }

    #[test]
    fn spatial_hashmap_when_object_at_boundaries_should_be_handled_correctly() {
        let map_width = 20.0.to_scalar();
        let map_height = 20.0.to_scalar();
        let grid_width = 2;
        let grid_height = 2;
        let mut shm = SpatialHashMap::new(map_width, map_height, grid_width, grid_height); // 10x10 cells

        let obj_id1 = 301;
        // AABB (0,0)-(10,10) - should touch cells (0,0), (0,1), (1,0), (1,1) if its max values are handled correctly
        let aabb1 = create_aabb(0.0, 0.0, 10.0, 10.0);
        shm.insert(obj_id1, &aabb1);
        assert!(
            shm.query(&create_aabb(0.0, 0.0, 0.0, 0.0))
                .contains(&obj_id1)
        ); // Bottom-left corner
        assert!(
            shm.query(&create_aabb(9.9, 9.9, 9.9, 9.9))
                .contains(&obj_id1)
        ); // Near cell (0,0)'s max
        assert!(
            shm.query(&create_aabb(10.0, 10.0, 10.0, 10.0))
                .contains(&obj_id1)
        ); // Edge where cells meet (10,10)

        let obj_id2 = 302;
        // AABB (10,10)-(20,20) - top-right cell
        let aabb2 = create_aabb(10.0, 10.0, 20.0, 20.0);
        shm.insert(obj_id2, &aabb2);
        assert!(
            shm.query(&create_aabb(19.9, 19.9, 19.9, 19.9))
                .contains(&obj_id2)
        ); // Near map max
        assert!(
            !shm.query(&create_aabb(0.0, 0.0, 0.0, 0.0))
                .contains(&obj_id2)
        ); // Not obj2's cell

        // Query an AABB covering the whole map
        let results_full_map = shm.query(&create_aabb(0.0, 0.0, 20.0, 20.0));
        assert!(results_full_map.contains(&obj_id1));
        assert!(results_full_map.contains(&obj_id2));
        assert_eq!(results_full_map.len(), 2);
    }

    #[test]
    fn spatial_hashmap_when_object_outside_boundaries_should_be_clamped() {
        let map_width = 20.0.to_scalar();
        let map_height = 20.0.to_scalar();
        let grid_width = 2;
        let grid_height = 2;
        let mut shm = SpatialHashMap::new(map_width, map_height, grid_width, grid_height);

        let obj_id1 = 401;
        // AABB slightly negative on min, slightly over map_width/height on max
        let aabb1 = create_aabb(-5.0, -5.0, 25.0, 25.0); // Effectively (0,0)-(20,20) due to clamping
        shm.insert(obj_id1, &aabb1);

        // Query the corner within bounds (should find it)
        assert!(
            shm.query(&create_aabb(0.0, 0.0, 0.0, 0.0))
                .contains(&obj_id1)
        );
        assert!(
            shm.query(&create_aabb(19.9, 19.9, 19.9, 19.9))
                .contains(&obj_id1)
        );

        // Query slightly outside (should still find it due to query's own clamping)
        assert!(
            shm.query(&create_aabb(-1.0, -1.0, 1.0, 1.0))
                .contains(&obj_id1)
        );
        assert!(
            shm.query(&create_aabb(19.0, 19.0, 21.0, 21.0))
                .contains(&obj_id1)
        );

        // Ensure keys_iter handles clamping correctly for indices
        let keys_for_aabb1: HashSet<u32> = shm.keys_iter(&aabb1).collect();
        let expected_keys_full_map: HashSet<u32> = (0..(grid_width * grid_height)).collect();
        assert_eq!(keys_for_aabb1, expected_keys_full_map);
    }

    #[test]
    fn spatial_hashmap_when_multiple_objects_in_same_cell_should_find_all_objects() {
        let mut shm = SpatialHashMap::new(100.0.to_scalar(), 100.0.to_scalar(), 10, 10);
        let obj_id1 = 501;
        let obj_id2 = 502;

        let aabb1 = create_aabb(1.0, 1.0, 9.0, 9.0); // Cell (0,0)
        let aabb2 = create_aabb(2.0, 2.0, 8.0, 8.0); // Also cell (0,0)

        shm.insert(obj_id1, &aabb1);
        shm.insert(obj_id2, &aabb2);

        let query_aabb = create_aabb(5.0, 5.0, 5.0, 5.0);
        let results = shm.query(&query_aabb);
        assert!(results.contains(&obj_id1));
        assert!(results.contains(&obj_id2));
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn spatial_hashmap_when_map_has_single_cell_should_find_all_objects_in_cell() {
        let map_width = 10.0.to_scalar();
        let map_height = 10.0.to_scalar();
        let grid_width = 1;
        let grid_height = 1;
        let mut shm = SpatialHashMap::new(map_width, map_height, grid_width, grid_height);

        let obj_id = 601;
        let aabb = create_aabb(1.0, 1.0, 9.0, 9.0);
        shm.insert(obj_id, &aabb);

        let results = shm.query(&create_aabb(5.0, 5.0, 5.0, 5.0));
        assert!(results.contains(&obj_id));
        assert_eq!(results.len(), 1);

        shm.clear();
        assert!(shm.query(&create_aabb(5.0, 5.0, 5.0, 5.0)).is_empty());
    }

    #[test]
    fn spatial_hashmap_keys_iter_when_aabb_aligned_with_boundaries_should_return_correct_cells() {
        let map_width = 20.0.to_scalar();
        let map_height = 20.0.to_scalar();
        let grid_width = 2; // Cells are 10x10
        let grid_height = 2;
        let shm = SpatialHashMap::new(map_width, map_height, grid_width, grid_height);

        // AABB (0,0)-(10,10)
        // should cover cells (0,0) and (0,1) for x, (0,0) and (1,0) for y
        let aabb = create_aabb(0.0, 0.0, 10.0, 10.0);
        let keys: HashSet<u32> = shm.keys_iter(&aabb).collect();
        // Cell indices: (0,0) -> 0, (1,0) -> 1, (0,1) -> 2, (1,1) -> 3
        let expected_keys: HashSet<u32> = [0, 1, 2, 3].iter().cloned().collect();
        assert_eq!(keys, expected_keys);

        // AABB (10,10)-(20,20)
        // cells are [0, 10), [10, 20] along both axes, so should only cover cell (1,1)
        let aabb_top_right = create_aabb(10.0, 10.0, 20.0, 20.0);
        let keys_tr: HashSet<u32> = shm.keys_iter(&aabb_top_right).collect();
        // (1,1) -> 3
        let expected_keys_tr: HashSet<u32> = [3].iter().cloned().collect();
        assert_eq!(keys_tr, expected_keys_tr);

        // AABB (9.9,9.9)-(10.1,10.1) should ideally cover 4 cells
        let aabb_cross = create_aabb(9.9, 9.9, 10.1, 10.1);
        let keys_cross: HashSet<u32> = shm.keys_iter(&aabb_cross).collect();
        // min_x_idx = 0 (floor(9.9/10)=0), max_x_idx = 1 (floor(10.1/10)=1)
        // min_y_idx = 0 (floor(9.9/10)=0), max_y_idx = 1 (floor(10.1/10)=1)
        // Expected keys are 0,1,2,3
        let expected_keys_cross: HashSet<u32> = [0, 1, 2, 3].iter().cloned().collect();
        assert_eq!(keys_cross, expected_keys_cross);
    }

    #[test]
    fn spatial_hashmap_when_object_at_max_edge_should_be_in_correct_cell() {
        let map_width = 100.0.to_scalar();
        let map_height = 100.0.to_scalar();
        let grid_width = 10; // Cells are 10x10
        let grid_height = 10;
        let mut shm = SpatialHashMap::new(map_width, map_height, grid_width, grid_height);

        let obj_id = 701;
        // AABB at the very top-right corner of the map.
        // min.x and min.y are slightly less than max boundary, max.x and max.y are exactly at map_width/height.
        let aabb = create_aabb(95.0, 95.0, 100.0, 100.0);
        shm.insert(obj_id, &aabb);

        // Query a point within the top-rightmost cell (index (grid_width-1, grid_height-1) which is (9,9))
        let query_aabb_top_right_cell = create_aabb(99.0, 99.0, 99.0, 99.0);
        let results = shm.query(&query_aabb_top_right_cell);
        assert!(results.contains(&obj_id));
        assert_eq!(results.len(), 1);

        // Ensure it's not found in an adjacent cell (e.g., (8,9))
        let query_aabb_adjacent_cell = create_aabb(85.0, 95.0, 85.0, 95.0);
        let results_adjacent = shm.query(&query_aabb_adjacent_cell);
        assert!(!results_adjacent.contains(&obj_id));

        // Verify keys_iter for this specific AABB
        let keys: HashSet<u32> = shm.keys_iter(&aabb).collect();
        // The object spans from (95,95) to (100,100).
        // min_x_idx = floor(95/10) = 9
        // min_y_idx = floor(95/10) = 9
        // max_x_idx = floor(100/10) = 10, clamped to 9
        // max_y_idx = floor(100/10) = 10, clamped to 9
        // So it should only be in cell (9,9) which has key 9 + 9*10 = 99
        let expected_keys: HashSet<u32> = [99].iter().cloned().collect();
        assert_eq!(keys, expected_keys);
    }
}
