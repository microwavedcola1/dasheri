```
programs
└── dasheri
    ├── Cargo.toml
    ├── Xargo.toml
    ├── src
    │   ├── ids.rs
    │   ├── iou # example of minting iou tokens when depositing to a mango account 
    │   │   ├── instructions
    │   │   ├── mod.rs
    │   │   └── state
    │   ├── lib.rs
    │   └── pool # example of pooling assets and creating a mango account representing the whole pool
    │       ├── error.rs
    │       ├── instructions
    │       ├── mod.rs
    │       └── state
    └── tests
        ├── fixtures
        │   ├── mango.so
        │   └── serum_dex.so
        ├── program_test
        │   ├── assertions.rs
        │   ├── cookies.rs
        │   ├── mod.rs
        │   └── scenarios.rs
        ├── test_iou.rs # test for iou tokens
        └── test_pool.rs # test for pool
```

The point of this anchor project is to serve as a starter kit or example to compose with mango-v3 using anchor. 
It currently provides 2 examples and various inline todos on how to extend this.

Use cases
* pool - pooled market making, pooled liquidator which can liquidate other large accounts
* iou - iou tokens via a gateway to mango's borrowing and lending

