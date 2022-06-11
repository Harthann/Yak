FROM alpine:3.16

RUN apk update; \
apk add build-base

WORKDIR /root

ENTRYPOINT ["ld"]
