use Mix.Config

config :mastery_persistance, MasteryPersistance.Repo,
  database: "mastery_test",
  hostname: "localhost",
  pool: Ecto.Adapters.SQL.Sandbox
