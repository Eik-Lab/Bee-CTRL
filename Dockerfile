FROM rustembedded/cross:armv7-unknown-linux-gnueabihf

RUN apt-get install libpq5 postgresql-all
