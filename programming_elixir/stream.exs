IO.inspect(
  [1, 2, 3, 4, 5]
  |> Stream.map(&(&1 * &1))
  |> Stream.map(&(&1 + 1))
  |> Stream.filter(&(rem(&1, 2) == 1))
  |> Enum.to_list()
)
