defmodule MyList do
  def len([]), do: 0
  def len([_head | tail]), do: 1 + len(tail)

  def square([]), do: []
  def square([head | tail]), do: [head * head | square(tail)]

  def add_1([]), do: []
  def add_1([head | tail]), do: [head + 1 | add_1(tail)]

  def map([], _func), do: []
  def map([head | tail], func), do: [func.(head) | map(tail, func)]

  def reduce([], acc, _func), do: acc
  def reduce([head | tail], acc, func), do: reduce(tail, func.(acc, head), func)

  def sum([]), do: 0
  def sum(l), do: reduce(l, 0, &(&1 + &2))

  def mapsum([], _func), do: 0
  def mapsum(list, func), do: list |> map(func) |> sum

  def max([head]), do: head
  def max([head, next | tail]) when head > next, do: max([head | tail])
  def max([_head, next | tail]), do: max([next | tail])

  def caesar([], _n), do: []
  def caesar([head | tail], n), do: [rem(head - 97 + n, 26) + 97 | caesar(tail, n)]

  def span(from, to) when from == to, do: [from]
  def span(from, to) when from < to, do: [from | span(from + 1, to)]

  def all?([], _predicate), do: true
  def all?([head | tail], predicate), do: predicate.(head) and all?(tail, predicate)

  def any?([], _predicate), do: false
  def any?([head | tail], predicate), do: predicate.(head) or any?(tail, predicate)

  def filter([], _predicate), do: []

  def filter([head | tail], predicate) do
    if predicate.(head) do
      [head | filter(tail, predicate)]
    else
      filter(tail, predicate)
    end
  end

  def split([], _n), do: {[], []}
  def split(list, 0), do: {[], list}
  def split(list, n) when n >= length(list), do: {list, []}
  def split(list, n), do: split_internal({[], list}, n)

  defp split_internal(pair, 0) do
    pair
  end

  defp split_internal({left = [], right = [rhead | rtail]}, curr) do
    split_internal({[rhead], rtail}, curr - 1)
  end

  defp split_internal({left, right = [rhead | rtail]}, curr) do
    split_internal({left ++ [rhead], rtail}, curr - 1)
  end

  def take([], _n), do: []
  def take(_list, 0), do: []
  def take([head | tail], n), do: [head | take(tail, n - 1)]
end
