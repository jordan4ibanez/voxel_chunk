# voxel_chunk

Voxel chunk is a very simple module which allows you to create "Chunk" objects.

These chunks are bound by a very simple contraints:

1. They are a fixed size.
2. They are 16 blocks wide, 16 blocks deep, and 128 blocks tall.
3. Blocks are defined as u32 for a LOT of blocks to be defined.
4. Height map is precalculated to be 256 long as it's 16x16 and u8.
5. Rotation and light are bound by u8, I would have used u4 if it were available.
6. Block, light, and rotation are precalculated to be 32768 as they're 16 * 128 * 16 in a 1D array, with methods to extract and inject data.
7. They are designed to be as light weight as possible, feel free to suggest any more performance boosting modifications.

### This is my first real crate so feel free to comment about what is wrong with it on Github.