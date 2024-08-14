# Basic comprehensions are like Enum.map/2
l = for x <- [1, 2, 3], do: x * x
IO.inspect(l)

# Comprehensions allow for nested loops
ll = for x <- [1, 2, 3], y <- [1, 2, 3], do: {x, y, x * y}
IO.inspect(ll)

# Comprehensions can loop over any enumerable
ll = for x <- 1..3, y <- 1..3, do: {x, y, x * y}
IO.inspect(ll)

# Comprehensions can return different *collectables*
multiplication_table =
  for x <- 1..3, y <- 1..3, into: %{} do
    # {key, value}
    {{x, y}, x * y}
  end

IO.inspect(multiplication_table)

# You can filter elements in comprehensions, the filter
# is executed for each element before the do..end block
# is executed.
multiplication_table =
  for x <- 1..3, y <- 1..3, x <= y, into: %{} do
    {{x, y}, x * y}
  end

IO.inspect(multiplication_table)

# Comprehensions have the same capabilities as the Enum
# functions, in some cases they are more elegant however.
