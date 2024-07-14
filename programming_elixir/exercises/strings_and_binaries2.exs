defmodule MyChars do
  def anagram?(word1, word2) when length(word1) != length(word2) do
    false
  end

  def anagram?(word1, word2) do
    sorted_word1 = Enum.sort(word1)
    sorted_word2 = Enum.sort(word2)

    [sorted_word1, sorted_word2]
    |> List.zip()

    ea |> Enum.all?(fn {a, b} -> a === b end)
  end
end
