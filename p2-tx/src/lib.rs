pub fn handle_tx(tx: &shared::Tx) {
    println!("[tx:{}] ops={}", tx.id, tx.ops.len());
    tracer::on_tx(tx);
}
