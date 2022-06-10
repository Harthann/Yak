FROM alpine:3.16

RUN apk update; \
apk add xorriso grub grub-bios

WORKDIR /root

ENTRYPOINT ["grub-mkrescue"]
