extern crate acto_rs as actors;
use actors::*;

#[allow(dead_code)]
struct EchoSource {}

impl source::Source for EchoSource {
  type OutputType = u64;

  fn process(&mut self, _output: &mut Sender<Message<Self::OutputType>>) -> Schedule {
    Schedule::OnExternalEvent
  }
}

#[allow(dead_code)]
struct EchoSink {}

impl sink::Sink for EchoSink {
  type InputType = u64;

  fn process(&mut self, input: &mut ChannelWrapper<Self::InputType>) -> Schedule {
    if let &mut ChannelWrapper::ConnectedReceiver(ref mut channel_id,
                                                  ref mut _receiver,
                                                  ref mut _sender_name) = input {
      Schedule::OnMessage(*channel_id)
    } else {
      Schedule::Loop
    }
  }
}

fn main() {
  use actors::connectable::Connectable;

  let mut sched = Scheduler::new();

  let (source_task, mut source_out) =
    source::new( "EchoSource", 2_000, Box::new(EchoSource{}));

  let mut sink_task =
    sink::new( "EchoSink", Box::new(EchoSink{}));

  sink_task.connect(&mut source_out).unwrap();

  let _source_id = sched.add_task(source_task).unwrap();
  let _sink_id = sched.add_task(sink_task);

  sched.start();
  sched.stop();

  println!("Hello, actors!");
}
