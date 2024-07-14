defmodule MyChars do
  def printable_ascii?(charlist) do
    Enum.all?(charlist, &(32 <= &1 and &1 <= 126))
  end
end
