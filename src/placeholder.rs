use actors::*;

#[allow(dead_code)]
pub struct Placeholder {}

impl source::Source for Placeholder {
  type OutputType = u64;

  fn process(&mut self, _output: &mut Sender<Message<Self::OutputType>>) -> Schedule {
    Schedule::Stop
  }
}
