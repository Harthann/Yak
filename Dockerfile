FROM alpine:3.16

# Deps
RUN apk update; \
apk add xorriso grub grub-bios

WORKDIR /root
