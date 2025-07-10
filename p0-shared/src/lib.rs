#[derive(Debug)]
pub struct Block {
    pub id: usize,
    pub txs: Vec<Tx>,
}

#[derive(Debug)]
pub struct Tx {
    pub id: usize,
    pub ops: Vec<Op>,
}

#[derive(Debug)]
pub struct Op(pub usize);
