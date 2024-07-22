defmodule SuperSequence.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, initial_number) do
    children = [
      {SuperSequence.Stash, initial_number},
      {SuperSequence.Server, nil}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :rest_for_one, name: SuperSequence.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
