pub fn handle_op(op: &shared::Op) {
    println!("[op] {op:?}");
    tracer::on_op(op);
}
