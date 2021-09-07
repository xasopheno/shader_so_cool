pub use weresocool::generation::json::Op4D;
type OpChannel = std::sync::mpsc::Receiver<Op4D>;
use crossbeam_channel::unbounded;

pub fn make_sender_and_receiver() -> crossbeam_channel::Receiver<Op4D> {
    let (op_tx, op_rx) = unbounded::<Op4D>();

    op_rx
}
