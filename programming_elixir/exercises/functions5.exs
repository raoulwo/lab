l = [1, 2, 3, 4]
l = Enum.map(l, &(&1 + 2))
Enum.map(l, &IO.inspect/1)
