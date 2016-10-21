use actors::*;

pub struct Parser {}

impl filter::Filter for Parser {
  type InputType = u64;
  type OutputType = u64;

  fn process(&mut self,
             _input:   &mut ChannelWrapper<Self::InputType>,
             _output:  &mut Sender<Message<Self::OutputType>>,
             _stop: &mut bool)
  {
  }
}
