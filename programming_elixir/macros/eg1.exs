defmodule My do
  defmacro macro(code) do
    IO.inspect(code)
    quote do: IO.puts("Hello, macro!")
  end
end

defmodule Test do
  require My

  My.macro(IO.inspect("Hello, world!"))
end
