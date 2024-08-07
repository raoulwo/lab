defmodule Tracer do
  defmacro def(definition, do: _content) do
    IO.inspect(definition)
    quote do: {}
  end
end

defmodule Test do
  import Kernel, except: [def: 2]
  import Tracer, only: [def: 2]

  def puts_sum_three(a, b, c), do: IO.inspect(a + b + c)
  def sum_list(list), do: Enum.reduce(list, 0, &(&1 + &2))
end

Test.puts_sum_three(1, 2, 3)
Test.sum_list([5, 6, 7, 8])
