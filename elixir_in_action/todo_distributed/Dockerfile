ARG ELIXIR="1.17.2"
ARG ERLANG="27.0.1"
ARG DEBIAN="bookworm-20240812-slim"
ARG OS="debian-${DEBIAN}"
FROM "hexpm/elixir:${ELIXIR}-erlang-${ERLANG}-${OS}" as builder

# NOTE: We don't take advantage of layer caching.

WORKDIR /todo

ENV MIX_ENV="prod"

RUN mix local.hex --force && mix local.rebar --force

COPY mix.exs mix.lock ./
COPY config config
COPY lib lib

RUN mix deps.get --only prod

RUN mix release

# NOTE: Use same base image as in previous stage.
FROM debian:${DEBIAN} as runner

WORKDIR /todo

RUN apt-get update -y && apt-get install -y openssl locales

COPY \
  --from=builder \
  --chown=nobody:root \
  /todo/_build/prod/rel/todo ./

RUN sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen && locale-gen

ENV LANG="en_US.UTF-8"
ENV LANGUAGE="en_US:en"
ENV LC_ALL="en_US.UTF-8"

CMD ["/todo/bin/todo", "start_iex"]
