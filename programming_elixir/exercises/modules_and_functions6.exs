defmodule Chop do
  def guess(actual, lo..hi//1) when lo <= hi and actual in lo..hi//1 do
    guess = div(lo + hi, 2)
    IO.puts("Is it #{guess}")
    guess(actual, guess, lo..hi)
  end

  defp guess(actual, guess, _) when actual == guess do
    IO.puts(actual)
  end

  defp guess(actual, guess, _..hi//1) when actual > guess do
    guess(actual, guess+1..hi)
  end

  defp guess(actual, guess, lo.._//1) when actual < guess do
    guess(actual, lo..guess-1)
  end
end
