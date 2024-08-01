defmodule RumblWeb.ChannelCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      use Phoenix.ChannelTest

      @endpoint RumblWeb.Endpoint
    end
  end

  setup tags do
    :ok = Ecto.Adapters.SQL.Sandbox.checkout(Rumbl.Repo)

    unless tags[:async] do
      Ecto.Adapters.SQL.Sandbox.mode(Rumbl.Repo, {:shared, self()})
    end

    :ok
  end
end
