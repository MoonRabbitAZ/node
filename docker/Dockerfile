FROM moonRabbitAZ/ci-linux:production as builder
LABEL description="This is the build stage for moonrabbit. Here we create the binary."

ARG PROFILE=release
WORKDIR /moonrabbit

COPY . /moonrabbit

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the moonrabbit binary."
ARG PROFILE=release
COPY --from=builder /moonrabbit/target/$PROFILE/moonrabbit /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /moonrabbit moonrabbit && \
	mkdir -p /moonrabbit/.local/share && \
	mkdir /data && \
	chown -R moonrabbit:moonrabbit /data && \
	ln -s /data /moonrabbit/.local/share/moonrabbit && \
	rm -rf /usr/bin /usr/sbin

USER moonrabbit
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/moonrabbit"]
