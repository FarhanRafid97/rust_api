FROM rust:1.78-buster AS base_deps

RUN apt update -y && apt upgrade -y
RUN apt install build-essential -y
RUN apt install lld clang -y

FROM base_deps AS build
ENV USER=app
ENV UID=10001
RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "${UID}" "${USER}"
WORKDIR /app
COPY . .
RUN cargo b -r

FROM gcr.io/distroless/cc-debian12 AS deployment

COPY --from=build /etc/passwd /etc/passwd
ENV DATABASE_URL=postgres://farhanpro@host.docker.internal/rust_api
COPY --from=build /etc/group /etc/group
WORKDIR /app
COPY --from=build /app/target/release/api-project ./
EXPOSE 8080
CMD ["./api-project"]