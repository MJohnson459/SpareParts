FROM debian:stretch@sha256:fd78f894f65bbdecbd9c533dffe3e6a1a092f5087462325639caa9afb5428f52

COPY ./target/armv7-unknown-linux-gnueabihf/release/spare-parts /
COPY ./scripts /scripts

CMD ["/spare-parts"]
