use actors::*;

pub struct HandleParsed {}

impl sink::Sink for HandleParsed {
  type InputType = u64;

  fn process(&mut self,
             input: &mut ChannelWrapper<Self::InputType>,
             _stop: &mut bool) {

    if let &mut ChannelWrapper::ConnectedReceiver(ref mut channel_id,
                                                  ref mut _receiver,
                                                  ref mut _sender_name) = input {
    }
  }
}
