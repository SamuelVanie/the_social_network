(dap-register-debug-template "Tauri debug conf"
                             (list :type "gdb"
                                   :request "launch"
                                   :gdbpath "rust-gdb"
                                   :dap-compilation "cargo build --manifest-path=./Cargo.toml --no-default-features"
                                   ))
