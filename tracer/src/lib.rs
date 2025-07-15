use std::io::Write;

#[derive(Debug, Default)]
pub struct State {
    pub traces: Vec<BlockTrace>,
}

#[derive(Debug, Default)]
pub struct BlockTrace {
    pub block_id: usize,
    pub txs: Vec<TxTrace>,
}

#[derive(Debug, Default)]
pub struct TxTrace {
    pub block_id: usize,
    pub tx_id: usize,
    pub ops: Vec<OpTrace>,
}

#[derive(Debug, Default)]
pub struct OpTrace {
    pub block_id: usize,
    pub tx_id: usize,
    pub val: usize,
}

mod hidden {
    use super::State;
    zucchero::init!(State, expose);
}

pub fn flush(mut w: impl Write) {
    hidden::expose(|state| {
        let s = format!("{state:#?}");
        w.write_all(s.as_bytes()).unwrap();
        *state = State::default();
        println!("[tracer] flush");
    });
}

pub fn on_block(block: &shared::Block) {
    let trace = BlockTrace {
        block_id: block.id,
        txs: Vec::with_capacity(block.txs.len()),
    };
    hidden::expose(|state| state.traces.push(trace));
}

pub fn on_tx(tx: &shared::Tx) {
    hidden::expose(|state| {
        let block: &mut BlockTrace = state.traces.last_mut().expect("current block");
        let trace = TxTrace {
            block_id: block.block_id,
            tx_id: tx.id,
            ops: Vec::with_capacity(tx.ops.len()),
        };
        block.txs.push(trace);
    });
}

pub fn on_op(op: &shared::Op) {
    hidden::expose(|state| {
        let block = state.traces.last_mut().expect("current block");
        let tx = block.txs.last_mut().expect("current tx");
        let trace = OpTrace {
            block_id: block.block_id,
            tx_id: tx.tx_id,
            val: op.0,
        };
        tx.ops.push(trace);
    });
}
