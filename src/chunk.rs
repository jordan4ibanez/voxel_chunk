use cgmath::Vector3;

/*
    Notes:

    Precalculated math here goes as follows:

    x -> 16 * 128 (2048)
    z -> 128
    y -> remainder

    block/rotation/light -> 16 * 128 * 16 (32768)

    heightmap -> 16 * 16 (256)

    This makes the Y values directly next to each other in the array.

    This has 3 major benefits:

    1.) Rapid scanning heightmaps is faster since memory pointers are right next to eachother.

    2.) Weather calculations are made much faster due to being able to in-fill with a Y scan downwards.

    3.) It is faster to generate chunk meshes when scanning bottom to top, as you will be running
        through the memory in a linear fashion with pointer jumps after moving to the next Z coordinate.

*/

pub struct Chunk {
    position:  [i64; 2],
    block:     [u32; 32768],
    rotation:  [u8;  32768],
    light:     [u8;  32768],
    heightmap: [u8;  256]
}


// Converts Vector3<u8> 3D position into u16 1D position.
pub fn vector_pos_to_index ( Vector3 { x, y, z }: &Vector3<u8> ) -> u16 {

    let x_wide: u16 = x.clone().into();
    let y_wide: u16 = y.clone().into();
    let z_wide: u16 = z.clone().into();

    (x_wide * 2048) + (z_wide * 128) + y_wide

}

// Converts x,y,z (u8) 3D position into u16 1D position.
pub fn pos_to_index ( x: &u8, y: &u8, z: &u8 ) -> u16 {

    let x_wide: u16 = x.clone().into();
    let y_wide: u16 = y.clone().into();
    let z_wide: u16 = z.clone().into();

    (x_wide * 2048) + (z_wide * 128) + y_wide

}

// Converts u16 1D position into Vector3<u8> 3D position.
pub fn index_to_vector_pos (i: &u16) -> Vector3<u8> {

    let mut index :u16 = i.clone();

    let x: u8 = (index / 2048).try_into().unwrap();

    index = index % 2048;

    let z: u8 = (index / 128).try_into().unwrap();

    index = index % 128;

    let y: u8 = index.try_into().unwrap();

    Vector3::new(x, y, z)

}

// Convertes u16 1D position into (u8,u8,u8) 3D tuple position
pub fn index_to_pos ( i: &u16 ) -> (u8,u8,u8) {

    let mut index :u16 = i.clone();

    let x: u8 = (index / 2048).try_into().unwrap();

    index = index % 2048;

    let z: u8 = (index / 128).try_into().unwrap();

    index = index % 128;

    let y: u8 = index.try_into().unwrap();

    (x, y, z)

}

// Converts 2D u8 positions into 1D u8 array position.
pub fn heightmap_pos_to_index ( x: &u8, z: &u8 ) -> u8 {
    (x * 16) + z
}

// Converts 1D u8 array position into 2D u8 tuple. (X and Z)
pub fn heightmap_index_to_pos ( i: &u8 ) -> (u8,u8) {

    let mut index: u8 = i.clone();

    let x: u8 = (index / 16).try_into().unwrap();

    index = index % 16;

    let z: u8 = index.try_into().unwrap();

    (x,z)
}

// Turns the extracted data into a new data pointer to be used as needed.
fn extract_u32 ( index: &u32 ) -> u32 {

    let value: u32 = index.clone();

    value
    
}

fn extract_u8( index: &u8 ) -> u8 {

    let value: u8 = index.clone();

    value
    
}

impl Chunk {

    // Position getter.
    pub fn get_pos (&self) -> (i64, i64) {
        (self.position[0].clone(), self.position[1].clone())
    }


    // Block data getters.

    pub fn get_block_from_index ( &self, index: &u16 ) -> u32 {

        let this_index: usize = index.clone().into();

        extract_u32(&self.block[this_index])

    }

    pub fn get_block_from_vector_position ( &self, position: &Vector3<u8> ) -> u32 {

        let this_index: usize = vector_pos_to_index(position).into();

        extract_u32(&self.block[this_index])

    }

    pub fn get_block_from_position ( &self, x: &u8, y: &u8, z: &u8 ) -> u32 {

        let this_index: usize = pos_to_index(x, y, z).into();

        extract_u32(&self.block[this_index])

    }

    // Rotation data getters.

    pub fn get_rotation_from_index ( &self, index: &u16 ) -> u8 {

        let this_index: usize = index.clone().into();

        extract_u8(&self.rotation[this_index])

    }

    pub fn get_rotation_from_vector_position ( &self, position: &Vector3<u8> ) -> u8 {

        let this_index: usize = vector_pos_to_index(position).into();

        extract_u8(&self.rotation[this_index])

    }

    pub fn get_rotation_from_position ( &self, x: &u8, y: &u8, z: &u8 ) -> u8 {

        let this_index: usize = pos_to_index(x, y, z).into();

        extract_u8(&self.rotation[this_index])

    }

    // Light data getters.

    pub fn get_light_from_index ( &self, index: &u16 ) -> u8 {

        let this_index: usize = index.clone().into();

        extract_u8(&self.light[this_index])

    }

    pub fn get_light_from_vector_position ( &self, position: &Vector3<u8> ) -> u8 {

        let this_index: usize = vector_pos_to_index(position).into();

        extract_u8(&self.light[this_index])
        
    }

    pub fn get_light_from_position ( &self, x: &u8, y: &u8, z: &u8 ) -> u8 {

        let this_index: usize = pos_to_index(x, y, z).into();

        extract_u8(&self.light[this_index])

    }

    // Heightmap data getters.

    pub fn get_heightmap_from_index ( &self, index: &u8 ) -> u8 {

        let this_index: usize = index.clone().into();

        extract_u8(&self.heightmap[this_index])

    }

    pub fn get_heightmap_from_pos ( &self, x: &u8, z: &u8 ) -> u8 {

        let this_index: usize = heightmap_pos_to_index(x, z).into();

        extract_u8(&self.heightmap[this_index])

    }


    // Setters from here on down.


    // Block data getters.

    pub fn set_block_from_index ( &mut self, index: &u16, block: &u32 ) {

        let this_index: usize = index.clone().into();

        self.block[this_index] = block.clone();

    }

    pub fn set_block_from_vector_position ( &mut self, position: &Vector3<u8> , block: &u32 ) {

        let this_index: usize = vector_pos_to_index(position).into();

        self.block[this_index] = block.clone();

    }

    pub fn set_block_from_position ( &mut self, x: &u8, y: &u8, z: &u8, block: &u32 ) {

        let this_index: usize = pos_to_index(x, y, z).into();

        self.block[this_index] = block.clone();

    }

    // Rotation data setters.

    pub fn set_rotation_from_index ( &mut self, index: &u16, rotation: &u8 ) {

        let this_index: usize = index.clone().into();

        self.rotation[this_index] = rotation.clone();

    }

    pub fn set_rotation_from_vector_position ( &mut self, position: &Vector3<u8>, rotation: &u8 ) {

        let this_index: usize = vector_pos_to_index(position).into();

        self.rotation[this_index] = rotation.clone();

    }

    pub fn set_rotation_from_position ( &mut self, x: &u8, y: &u8, z: &u8, rotation: &u8 ) {

        let this_index: usize = pos_to_index(x, y, z).into();

        self.rotation[this_index] = rotation.clone();

    }

    // Light data getters.

    pub fn set_light_from_index ( &mut self, index: &u16, light: &u8 ) {

        let this_index: usize = index.clone().into();

        self.light[this_index] = light.clone();

    }

    pub fn set_light_from_vector_position ( &mut self, position: &Vector3<u8>, light: &u8 ) {

        let this_index: usize = vector_pos_to_index(position).into();

        self.light[this_index] = light.clone();
        
    }

    pub fn set_light_from_position ( &mut self, x: &u8, y: &u8, z: &u8, light: &u8 ) {

        let this_index: usize = pos_to_index(x, y, z).into();

        self.light[this_index] = light.clone();

    }

    // Heightmap data getters.

    pub fn set_heightmap_from_index ( &mut self, index: &u8, height: &u8 ) {

        let this_index: usize = index.clone().into();

        self.heightmap[this_index] = height.clone();

    }

    pub fn set_heightmap_from_pos ( &mut self, x: &u8, z: &u8, height: &u8 ) {

        let this_index: usize = heightmap_pos_to_index(x, z).into();

        self.heightmap[this_index] = height.clone();

    }

}


// Creates a new Chunk "Object", positions x and z
pub fn new ( x: i64, z: i64 ) -> Chunk {
    Chunk {
        position:  [x,z],
        block:     [0; 32768],
        rotation:  [0;  32768],
        light:     [0;  32768],
        heightmap: [0;  256]
    }
}