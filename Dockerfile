FROM rustembedded/cross:armv7-unknown-linux-gnueabihf

RUN apt-get update && \
    apt-get install --assume-yes libpq5 libpq-dev
