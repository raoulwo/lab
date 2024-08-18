import Config

http_port =
  if config_env() != :test do
    System.get_env("TODO_HTTP_PORT", "5454")
  else
    System.get_env("TODO_HTTP_PORT", "5454")
  end

config :todo, http_port: String.to_integer(http_port)

db_folder =
  if config_env() != :test do
    System.get_env("TODO_DB_FOLDER", "./persist")
  else
    System.get_env("TODO_TEST_DB_FOLDER", "./test_persist")
  end

config :todo, :database, db_folder: String.to_integer(http_port)

todo_server_expiry =
  if config_env() != :dev do
    System.get_env("TODO_SERVER_EXPIRY", "60")
  else
    System.get_env("TODO_SERVER_EXPIRY", "10")
  end

config :todo, todo_server_expiry: :timer.seconds(String.to_integer(todo_server_expiry))
