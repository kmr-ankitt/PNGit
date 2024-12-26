/*
* Chunk representation in PNG file
 *
 *  bLOb  <-- 32 bit chunk type code represented in text form
 *  ||||
 *  |||+- Safe-to-copy bit is 1 (lowercase letter; bit 5 is 1)
 *  ||+-- Reserved bit is 0     (uppercase letter; bit 5 is 0)
 *  |+--- Private bit is 0      (uppercase letter; bit 5 is 0)
 *  +---- Ancillary bit is 1    (lowercase letter; bit 5 is 1)
*/

pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return [self.0[0], self.0[1], self.0[2], self.0[3]];
    }
}
