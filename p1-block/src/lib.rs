pub fn handle_block(block: &shared::Block) {
    println!("[block:{}] txs={}", block.id, block.txs.len());
    tracer::on_block(block);
}
