FROM alpine:3.16

RUN echo "x86" > /etc/apk/arch; \
apk update; \
apk add  build-base

WORKDIR /root

ENTRYPOINT ["ld"]
