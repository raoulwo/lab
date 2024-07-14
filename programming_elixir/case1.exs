defmodule Users do
  dave = %{ name: "Raoul", country: "Austria", likes: "programming" }
  case dave do
    %{country: some_country} = person ->
      IO.puts("#{person.name} lives in #{some_country}.")
    _ ->
      IO.puts("No matches.")
  end
end
