llvm-profdata-11 merge integration/cov/*.profraw -o merged.profdata
llvm-cov show target/debug/kvs-server \
    -Xdemangler=rustfilt \
    -instr-profile=merged.profdata \
    -show-line-counts-or-regions \
    -show-instantiations \
    --ignore-filename-regex="(.cargo|rustc)"
