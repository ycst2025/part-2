ARG VARIANT="trixie"
FROM rust:1-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends wget ruby build-essential gcc-arm-none-eabi gdb-multiarch qemu-system-arm
