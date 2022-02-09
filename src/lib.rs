pub mod chunk;

/*
Make sure to use:

cargo test chunk_test -- --show-output

For more detail on tests.
*/

#[cfg(test)]
mod tests {
    use cgmath::Vector3;
    use rand::Rng;

    use crate::chunk::{new, pos_to_index, index_to_pos, index_to_vector_pos, vector_pos_to_index, heightmap_index_to_pos, heightmap_pos_to_index};

    #[test]
    fn chunk_test() {

        println!("--- TEST STARTS HERE ---");

        let mut rng = rand::thread_rng();

        let mut chunk_test = new(rng.gen_range(-255..255), rng.gen_range(-255..255));

        // Fill the chunk with random data
        for i in 0..32768 {
            chunk_test.set_block_from_index(&i, &rng.gen_range(0..255));
            chunk_test.set_light_from_index(&i, &rng.gen_range(0..255));
            chunk_test.set_rotation_from_index(&i, &rng.gen_range(0..255));
        }
        for i in 0..=255 {
            chunk_test.set_heightmap_from_index(&i, &rng.gen_range(0..255));
        }


        // Check 2D (heightmap)
        for i in 0..=255 {
            let height_map_pos = heightmap_index_to_pos(&i);
            let height_map_index = heightmap_pos_to_index(&height_map_pos.0, &height_map_pos.1);

            assert_eq!(i, height_map_index);

            // Optional print to test indexing
            if true {
                println!("index: {} | calculated: {} | heightmap pos: {}, {}", i, height_map_index, height_map_pos.0, height_map_pos.1);
            }
        }


        // Now test 3D
        for i in 0..32768 {

            // Check the 1D to 3D

            let tuple_pos: (u8,u8,u8) = index_to_pos(&i);
            let vector_pos: Vector3<u8> = index_to_vector_pos(&i);

            assert_eq!(tuple_pos.0, vector_pos.x);
            assert_eq!(tuple_pos.1, vector_pos.y);
            assert_eq!(tuple_pos.2, vector_pos.z);

            // Check 3D to 1D and double check against index reconversion

            let vector_index = vector_pos_to_index(&vector_pos);
            let tuple_index = pos_to_index(&tuple_pos.0, &tuple_pos.1, &tuple_pos.2);

            assert_eq!(vector_index, tuple_index);
            assert_eq!(vector_index, i);
            assert_eq!(tuple_index, i);

            // Check all getters

            // Blocks

            let index_block_check = chunk_test.get_block_from_index(&i);
            let tuple_block_check = chunk_test.get_block_from_position(&tuple_pos.0, &tuple_pos.1, &tuple_pos.2);
            let vector_block_check = chunk_test.get_block_from_vector_position(&vector_pos);

            assert_eq!(index_block_check, tuple_block_check);
            assert_eq!(index_block_check, vector_block_check);
            assert_eq!(tuple_block_check, vector_block_check);

            // Rotation

            let index_rot_check = chunk_test.get_rotation_from_index(&i);
            let tuple_rot_check = chunk_test.get_rotation_from_position(&tuple_pos.0, &tuple_pos.1, &tuple_pos.2);
            let vector_rot_check = chunk_test.get_rotation_from_vector_position(&vector_pos);

            assert_eq!(index_rot_check, tuple_rot_check);
            assert_eq!(index_rot_check, vector_rot_check);
            assert_eq!(tuple_rot_check, vector_rot_check);

            // Light

            let index_light_check = chunk_test.get_light_from_index(&i);
            let tuple_light_check = chunk_test.get_light_from_position(&tuple_pos.0, &tuple_pos.1, &tuple_pos.2);
            let vector_light_check = chunk_test.get_light_from_vector_position(&vector_pos);

            assert_eq!(index_light_check, tuple_light_check);
            assert_eq!(index_light_check, vector_light_check);
            assert_eq!(tuple_light_check, vector_light_check);

            // Optional print to test mutability
            if true {
                println!("INDEX: {} -----------------------------", i);
                println!("B: {} | R: {} | L: {}", index_block_check, index_rot_check, index_light_check);
                println!("B: {} | R: {} | L: {}", tuple_block_check, tuple_rot_check, tuple_light_check);
                println!("B: {} | R: {} | L: {}", vector_block_check, vector_rot_check, vector_light_check);
            }
        }
    }
}
