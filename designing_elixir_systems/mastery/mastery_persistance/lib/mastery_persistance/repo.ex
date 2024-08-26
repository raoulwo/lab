defmodule MasteryPersistance.Repo do
  use Ecto.Repo,
    otp_app: :mastery_persistance,
    adapter: Ecto.Adapters.Postgres
end
