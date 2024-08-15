defmodule Fraction do
  # Define a struct associated with this module
  defstruct a: nil, b: nil

  # NOTE: By default structs aren't enumerable like maps

  def new(a, b), do: %Fraction{a: a, b: b}

  def value(%Fraction{a: a, b: b}), do: a / b

  def add(%Fraction{a: a1, b: b1}, %Fraction{a: a2, b: b2}) do
    new(a1 * b2 + a2 * b1, b1 * b2)
  end
end

f = Fraction.new(1, 2)

# Structs are maps with a `__struct__` field containing their
# associated module
f
|> Map.to_list()
|> IO.inspect()

# Because of the `__struct__` field, maps can't match against
# struct patterns. Structs can however match against maps.

# MatchError because the right hand side map doesn't contain
# the field `__struct__`
%Fraction{a: 1, b: 2} = %{a: 1, b: 2}
# Works because the right hand side struct contains all fields
# of the left hand side map
%{a: 1, b: 2} = %Fraction{a: 1, b: 2}

# NOTE: Aside from structs Elixir also has records which provide
# a way to access fields of tuples by name. Records exist primarily
# for historical reasons since they were used as the main tool for
# structured data before maps were used. You'll probably only need
# to use records for interaction with certain Erlang APIs.

# NOTE: IO.inspect/1 outputs the inspected data representation
# and returns the data itself making it useful for debugging inside
# pipelines
f =
  Fraction.new(1, 4)
  |> IO.inspect()
  |> Fraction.add(Fraction.new(1, 4))
  |> IO.inspect()
  |> Fraction.add(Fraction.new(1, 2))
  |> IO.inspect()
