# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd full-moon/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv full-moon/fuzz/target/x86_64-unknown-linux-gnu/release/roundtrip /roundtrip && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd full-moon/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv full-moon/fuzz/target/release/roundtrip /roundtrip_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /roundtrip /roundtrip_no_inst /
RUN mkdir /testsuite
COPY --from=builder /src/full-moon/tests/cases/pass/trivia-parsing/source.lua  /testsuite/
