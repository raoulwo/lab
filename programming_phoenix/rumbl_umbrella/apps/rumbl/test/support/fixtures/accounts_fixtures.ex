defmodule Rumbl.AccountsFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `Rumbl.Accounts` context.
  """

  alias Rumbl.Accounts

  @doc """
  Generate a user.
  """
  def user_fixture(attrs \\ %{}) do
    {:ok, user} =
      attrs
      |> Enum.into(%{
        name: "Some User",
        username: "user#{System.unique_integer([:positive])}",
        password: attrs[:password] || "supersecret"
      })
      |> Accounts.register_user()

    user
  end
end
