defmodule Loops do
  def list_len(list) when is_list(list), do: _list_len(0, list)
  defp _list_len(final_len, []), do: final_len
  defp _list_len(curr_len, [_head | tail]), do: _list_len(curr_len + 1, tail)

  def range(from, to) when is_integer(from) and is_integer(to) and from <= to do
    _range([], from, to)
  end

  defp _range(final_list, from, to) when from == to do
    [from | final_list]
  end

  defp _range(curr_list, from, to) do
    _range([to | curr_list], from, to - 1)
  end

  def positive(list) when is_list(list), do: _positive([], list)

  defp _positive(final_list, []), do: Enum.reverse(final_list)

  defp _positive(curr_list, [head | tail]) when head >= 0 do
    _positive([head | curr_list], tail)
  end

  defp _positive(curr_list, [head | tail]) when head < 0 do
    _positive(curr_list, tail)
  end
end
