defmodule MyString do
  def center([]), do: []

  def center(l) do
    column_width =
      l
      |> Enum.max_by(&String.length/1)
      |> String.length()

    Enum.each(l, fn
      str ->
        {x, y} = divrem(column_width - String.length(str), 2)
        left_pad = x
        right_pad = x + y

        str
        |> String.pad_leading(String.length(str) + left_pad)
        |> String.pad_trailing(String.length(str) + right_pad)
        |> IO.puts()
    end)
  end

  defp divrem(x, y) do
    {div(x, y), rem(x, y)}
  end
end
