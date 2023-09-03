#### Start a File Provider: In one terminal, run the following command to start a file provider node:

```
cargo run -- --listen-address /ip4/127.0.0.1/tcp/40837 \
          --secret-key-seed 1 \
          provide \
          --path <path-to-your-file> \
          --name <name-for-others-to-find-your-file>
```

#### Start a File Retriever: In another terminal, run the following command to start a file retriever node:

```
cargo run -- --peer /ip4/127.0.0.1/tcp/40837/p2p/12D3KooWPjceQrSwdWXPyLLeABRXmuqt69Rg3sBYbU1Nft9HyQ6X \
          get \
          --name <name-for-others-to-find-your-file>
```
