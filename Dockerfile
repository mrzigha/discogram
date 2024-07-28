FROM rust:slim-bookworm
LABEL name="Discogram"
LABEL maintainer="gagnieux.virgil@proton.me"
LABEL authors="MrZigha"
LABEL version="0.1.0"
LABEL license="MIT"
LABEL description="This image is used to run Discogram."

ARG PACKAGES="build-essential ca-certificates libssl-dev pkg-config"
ARG UID=10000
ARG GID=10000
ARG USERNAME=discogram

WORKDIR /build
COPY . .
RUN apt-get update && apt-get install -y $PACKAGES
RUN cargo build --release
RUN mv /build/target/release/discogram /bin
RUN groupadd -g ${GID} ${USERNAME} && useradd -u ${UID} -g ${GID} -m ${USERNAME}

USER ${UID}

WORKDIR /home/${USERNAME}

ENTRYPOINT [ "/bin/discogram" ]
