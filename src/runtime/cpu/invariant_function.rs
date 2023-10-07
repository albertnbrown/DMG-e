#[derive(Clone, Copy, Debug)]
pub enum InvariantFunction {
  F00, F08, F10, F18, F20, F28, F30, F38,
}

impl std::convert::From<InvariantFunction> for u16 {
  fn from(function: InvariantFunction) -> Self {
    match function {
      InvariantFunction::F00 => {
          return 0x0000_u16;
      }
      InvariantFunction::F08 => {
          return 0x0008_u16;
      }
      InvariantFunction::F10 => {
          return 0x0010_u16;
      }
      InvariantFunction::F18 => {
          return 0x0018_u16;
      }
      InvariantFunction::F20 => {
          return 0x0020_u16;
      }
      InvariantFunction::F28 => {
          return 0x0028_u16;
      }
      InvariantFunction::F30 => {
          return 0x0030_u16;
      }
      InvariantFunction::F38 => {
          return 0x0038_u16;
      }
    }
  }
}
