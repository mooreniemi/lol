llvm-profdata-11 merge integration/cov/*.profraw -o merged.profdata
llvm-cov-11 report target/debug/kvs-server \
    -Xdemangler=rustfilt \
    -instr-profile=merged.profdata \
    --ignore-filename-regex="(.cargo|rustc)"