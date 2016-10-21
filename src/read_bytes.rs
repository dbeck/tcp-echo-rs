use actors::*;

pub struct ReadBytes {}

impl source::Source for ReadBytes {
  type OutputType = u64;

  fn process(&mut self,
             _output: &mut Sender<Message<Self::OutputType>>,
             _stop: &mut bool)
  {
  }
}
