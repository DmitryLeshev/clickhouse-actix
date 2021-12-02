FROM debian
WORKDIR /app
ADD target/release/cliclhouse-rust .
CMD [ "/app/cliclhouse-rust" ]
