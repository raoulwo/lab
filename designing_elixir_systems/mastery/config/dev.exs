use Mix.Config

config :mastery_persistance, MasteryPersistance.Repo,
  database: "mastery_dev",
  hostname: "localhost"

config :mastery, :persistence_fn, &MasteryPersistance.record_response/2
