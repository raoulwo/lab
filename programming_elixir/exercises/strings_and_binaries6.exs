defmodule MyString do
  def capitalize_sentence(""), do: ""

  def capitalize_sentence(str) do
    str
    |> String.split(". ")
    |> Enum.map(&String.capitalize/1)
    |> Enum.join(". ")
  end
end

"oh. a DOG. woof. "
|> MyString.capitalize_sentence()
|> IO.puts()
