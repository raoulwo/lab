import Config

http_port =
  if config_env() != :test do
    System.get_env("TODO_HTTP_PORT", "5454")
  else
    System.get_env("TODO_HTTP_PORT", "5454")
  end

config :todo, http_port: String.to_integer(http_port)