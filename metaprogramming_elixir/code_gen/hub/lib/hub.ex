defmodule Hub do
  HTTPoison.start()

  @username "raoulwo"

  {:ok, response} =
    "https://api.github.com/users/#{@username}/repos"
    |> HTTPoison.get("User-Agent": "Elixir")

  response
  |> Map.get(:body)
  |> Poison.decode!()
  |> Enum.each(fn repo ->
    # We use `unquote fragments` to dynamically create
    # functions for each repo we fetched.
    def unquote(String.to_atom(repo["name"]))() do
      # `Macro.escape` takes an Elixir literal and recursively
      # escapes it for injection into an AST. It is required
      # anytime you need to inject Elixir values into an expression
      # that's already quoted (like inside `def`) where the value
      # is not an AST literal.
      unquote(Macro.escape(repo))
    end
  end)

  def go(repo) do
    url = apply(__MODULE__, repo, [])["html_url"]
    IO.puts("Launching browser to #{url}...")
    System.cmd("open", [url])
  end
end
