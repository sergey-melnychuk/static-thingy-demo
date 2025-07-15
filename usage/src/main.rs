use shared::{Block, Op, Tx};

fn blocks() -> Vec<Block> {
    let mut ret = Vec::new();
    for b in [100, 200, 300] {
        let mut block = Block {
            id: b,
            txs: Vec::with_capacity(3),
        };
        for t in [10, 20, 30] {
            let mut tx = Tx {
                id: b + t,
                ops: Vec::with_capacity(3),
            };
            for o in [1, 2, 3] {
                let op = Op(b + t + o);
                tx.ops.push(op);
            }
            block.txs.push(tx);
        }
        ret.push(block);
    }
    ret
}

fn main() {
    let blocks = blocks();

    for block in &blocks {
        block::handle_block(block);
        for tx in &block.txs {
            tx::handle_tx(tx);
            for op in &tx.ops {
                op::handle_op(op);
            }
        }
    }

    let mut w = std::io::BufWriter::new(Vec::new());
    tracer::flush(&mut w);

    let b = w.into_inner().unwrap();
    let s = String::from_utf8(b).unwrap();
    println!("{s}");
}
