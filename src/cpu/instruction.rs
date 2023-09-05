pub enum Instruction {
    ADD(RegisterTarget), // add register target to register a
}
  
pub enum RegisterTarget {
    A, B, C, D, E, H, L,
}