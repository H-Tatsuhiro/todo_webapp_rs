FROM rust:1.75.0 as rs
LABEL authors="privath"

WORKDIR /usr/src/myapp
COPY . .
COPY .sqlx .sqlx

RUN cargo build --release

FROM fedora:39 as fd39

RUN dnf update -y && dnf install nginx -y

COPY --from=rs /usr/src/myapp/target/release/todo_webapp_rs /usr/local/bin/myapp

COPY proxy.conf /etc/nginx/default.d/proxy.conf
EXPOSE 80

COPY run.sh run.sh
RUN chmod a+x run.sh
ENTRYPOINT ./run.sh