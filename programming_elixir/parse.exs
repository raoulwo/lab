defmodule Parse do
  def number([?- | tail]), do: -1 * _number_digits(tail, 0)
  def number([?+ | tail]), do: _number_digits(tail, 0)
  def number(str), do: _number_digits(str, 0)

  defp _number_digits([], value), do: value

  defp _number_digits([digit | tail], value) when digit in ~c"1234567890" do
    _number_digits(tail, 10 * value + digit - ?0)
  end

  defp _number_digits([non_digit | _tail], _value) do
    raise "Invalid digit '#{[non_digit]}'"
  end
end
