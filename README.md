static-thingy-demo
==================

Use static thingy for tracing.
Plug it in anywhere anytime.
It will work.

```
[block:100] txs=3
[tx:110] ops=3
[op] Op(111)
[op] Op(112)
[op] Op(113)
[tx:120] ops=3
[op] Op(121)
[op] Op(122)
[op] Op(123)
[tx:130] ops=3
[op] Op(131)
[op] Op(132)
[op] Op(133)
[block:200] txs=3
[tx:210] ops=3
[op] Op(211)
[op] Op(212)
[op] Op(213)
[tx:220] ops=3
[op] Op(221)
[op] Op(222)
[op] Op(223)
[tx:230] ops=3
[op] Op(231)
[op] Op(232)
[op] Op(233)
[block:300] txs=3
[tx:310] ops=3
[op] Op(311)
[op] Op(312)
[op] Op(313)
[tx:320] ops=3
[op] Op(321)
[op] Op(322)
[op] Op(323)
[tx:330] ops=3
[op] Op(331)
[op] Op(332)
[op] Op(333)
[tracer] flush
State {
    traces: [
        BlockTrace {
            block_id: 100,
            txs: [
                TxTrace {
                    block_id: 100,
                    tx_id: 110,
                    ops: [
                        OpTrace {
                            block_id: 100,
                            tx_id: 110,
                            val: 111,
                        },
                        OpTrace {
                            block_id: 100,
                            tx_id: 110,
                            val: 112,
                        },
                        OpTrace {
                            block_id: 100,
                            tx_id: 110,
                            val: 113,
                        },
                    ],
                },
                TxTrace {
                    block_id: 100,
                    tx_id: 120,
                    ops: [
                        OpTrace {
                            block_id: 100,
                            tx_id: 120,
                            val: 121,
                        },
                        OpTrace {
                            block_id: 100,
                            tx_id: 120,
                            val: 122,
                        },
                        OpTrace {
                            block_id: 100,
                            tx_id: 120,
                            val: 123,
                        },
                    ],
                },
                TxTrace {
                    block_id: 100,
                    tx_id: 130,
                    ops: [
                        OpTrace {
                            block_id: 100,
                            tx_id: 130,
                            val: 131,
                        },
                        OpTrace {
                            block_id: 100,
                            tx_id: 130,
                            val: 132,
                        },
                        OpTrace {
                            block_id: 100,
                            tx_id: 130,
                            val: 133,
                        },
                    ],
                },
            ],
        },
        BlockTrace {
            block_id: 200,
            txs: [
                TxTrace {
                    block_id: 200,
                    tx_id: 210,
                    ops: [
                        OpTrace {
                            block_id: 200,
                            tx_id: 210,
                            val: 211,
                        },
                        OpTrace {
                            block_id: 200,
                            tx_id: 210,
                            val: 212,
                        },
                        OpTrace {
                            block_id: 200,
                            tx_id: 210,
                            val: 213,
                        },
                    ],
                },
                TxTrace {
                    block_id: 200,
                    tx_id: 220,
                    ops: [
                        OpTrace {
                            block_id: 200,
                            tx_id: 220,
                            val: 221,
                        },
                        OpTrace {
                            block_id: 200,
                            tx_id: 220,
                            val: 222,
                        },
                        OpTrace {
                            block_id: 200,
                            tx_id: 220,
                            val: 223,
                        },
                    ],
                },
                TxTrace {
                    block_id: 200,
                    tx_id: 230,
                    ops: [
                        OpTrace {
                            block_id: 200,
                            tx_id: 230,
                            val: 231,
                        },
                        OpTrace {
                            block_id: 200,
                            tx_id: 230,
                            val: 232,
                        },
                        OpTrace {
                            block_id: 200,
                            tx_id: 230,
                            val: 233,
                        },
                    ],
                },
            ],
        },
        BlockTrace {
            block_id: 300,
            txs: [
                TxTrace {
                    block_id: 300,
                    tx_id: 310,
                    ops: [
                        OpTrace {
                            block_id: 300,
                            tx_id: 310,
                            val: 311,
                        },
                        OpTrace {
                            block_id: 300,
                            tx_id: 310,
                            val: 312,
                        },
                        OpTrace {
                            block_id: 300,
                            tx_id: 310,
                            val: 313,
                        },
                    ],
                },
                TxTrace {
                    block_id: 300,
                    tx_id: 320,
                    ops: [
                        OpTrace {
                            block_id: 300,
                            tx_id: 320,
                            val: 321,
                        },
                        OpTrace {
                            block_id: 300,
                            tx_id: 320,
                            val: 322,
                        },
                        OpTrace {
                            block_id: 300,
                            tx_id: 320,
                            val: 323,
                        },
                    ],
                },
                TxTrace {
                    block_id: 300,
                    tx_id: 330,
                    ops: [
                        OpTrace {
                            block_id: 300,
                            tx_id: 330,
                            val: 331,
                        },
                        OpTrace {
                            block_id: 300,
                            tx_id: 330,
                            val: 332,
                        },
                        OpTrace {
                            block_id: 300,
                            tx_id: 330,
                            val: 333,
                        },
                    ],
                },
            ],
        },
    ],
}
```