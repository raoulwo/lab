defmodule Utf8 do
  def each(str, func) when is_binary(str), do: _each(str, func)

  defp _each(<<>>, _func), do: []

  defp _each(<<head::utf8, tail::binary>>, func) do
    func.(head)
    _each(tail, func)
  end
end

Utf8.each("Grüß Gott!", fn char -> IO.puts(char) end)
