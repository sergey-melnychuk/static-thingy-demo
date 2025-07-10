use std::{io::Write, sync::Mutex};

use once_cell::sync::OnceCell;

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

static STATE: OnceCell<Mutex<State>> = OnceCell::new();

pub fn flush(mut w: impl Write) {
    let state = STATE.get_or_init(|| Mutex::new(State::default()));
    let mut guard = state.lock().unwrap();
    let state = format!("{:#?}", guard);
    w.write_all(state.as_bytes()).unwrap();
    *guard = State::default();
    println!("[tracer] flush");
}

pub fn on_block(block: &shared::Block) {
    let state = STATE.get_or_init(|| Mutex::new(State::default()));
    let mut guard = state.lock().unwrap();

    let trace = BlockTrace { block_id: block.id, txs: Vec::with_capacity(block.txs.len()) };
    (*guard).traces.push(trace);
}

pub fn on_tx(tx: &shared::Tx) {
    let state = STATE.get_or_init(|| Mutex::new(State::default()));
    let mut guard = state.lock().unwrap();
    
    let block: &mut BlockTrace = guard.traces.last_mut().expect("current block");
    let trace = TxTrace { block_id: block.block_id, tx_id: tx.id, ops: Vec::with_capacity(tx.ops.len()) };
    block.txs.push(trace);
}

pub fn on_op(op: &shared::Op) {
    let state = STATE.get_or_init(|| Mutex::new(State::default()));
    let mut guard = state.lock().unwrap();

    let block = guard.traces.last_mut().expect("current block");
    let tx = block.txs.last_mut().expect("current tx");
    let trace = OpTrace { block_id: block.block_id, tx_id: tx.tx_id, val: op.0 };
    tx.ops.push(trace);
}
