extern crate acto_rs as actors;
extern crate mio;
use actors::*;

mod listener;
mod placeholder;
mod read_bytes;
mod parser;
mod handle_parsed;

use read_bytes::ReadBytes;
use parser::Parser;
use handle_parsed::HandleParsed;

fn main() {
  use actors::connectable::Connectable;

  let mut sched = Scheduler::new();

  let (source_task, mut source_out) =
    source::new( "Read Bytes", 2_000, Box::new(ReadBytes{}));

  let (mut filter_task, mut filter_out) =
    filter::new( "Parser", 2_000, Box::new(Parser{}));

  let mut sink_task =
    sink::new( "Handle Parsed", Box::new(HandleParsed{}));

  filter_task.connect(&mut source_out).unwrap();
  sink_task.connect(&mut filter_out).unwrap();

  let _source_id = sched.add_task(source_task, actors::SchedulingRule::OnExternalEvent);
  let _sink_id = sched.add_task(sink_task, actors::SchedulingRule::OnMessage);

  sched.start();
  sched.stop();

  println!("Hello, actors!");
}
