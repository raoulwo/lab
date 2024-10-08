defmodule MasteryPersistance.MixProject do
  use Mix.Project

  def project do
    [
      app: :mastery_persistance,
      version: "0.1.0",
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {MasteryPersistance.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:ecto_sql, "~> 3.12.0"},
      {:postgrex, "~> 0.19.1"}
    ]
  end
end
