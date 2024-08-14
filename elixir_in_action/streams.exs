# Streams are lazy enumerables that are computed
# only on demand
stream = Stream.map([1, 2, 3], &(&1 * &1))
IO.inspect(stream)

# At this point the iteration of the stream is performed
l = Enum.to_list(stream)
IO.inspect(l)

employees = ["Alice", "Bob", "Eve"]

# Transformation of the stream without creating a new list
# before processing all elements
employees
|> Stream.with_index()
|> Enum.each(fn {employee, index} ->
  IO.puts("#{index + 1}. #{employee}")
end)

# Iteration is only done once, this is especially useful
# when several collection transformations are composed.
# When done using Enum functions multiple iterations
# are performed once for each intermediate list, leading
# to a memory overhead.
[9, -1, "foo", 25, 49]
|> Stream.filter(&(is_number(&1) and &1 > 0))
|> Stream.map(&{&1, :math.sqrt(&1)})
|> Stream.with_index()
|> Enum.each(fn {{input, result}, index} ->
  IO.puts("#{index + 1}. sqrt(#{input}) = #{result}")
end)

# Functions like Stream.iterate/1 and Stream.repeatedly/1
# can be used to generate infinite sequences
natural_numbers = Stream.iterate(1, &(&1 + 1))
IO.inspect(Enum.take(natural_numbers, 10))
