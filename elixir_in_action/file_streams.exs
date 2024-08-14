defmodule FileStreams do
  def lines_lengths!(path) do
    File.stream!(path)
    |> Stream.map(&String.trim_trailing(&1, "\n"))
    |> Enum.map(&String.length/1)
  end

  def longest_line_length!(path) do
    File.stream!(path)
    |> Stream.map(&String.trim_trailing(&1, "\n"))
    |> Stream.map(&String.length/1)
    |> Enum.max()
  end

  def longest_line!(path) do
    File.stream!(path)
    |> Stream.map(&String.trim_trailing(&1, "\n"))
    |> Enum.max(&(String.length(&1) >= String.length(&2)))
  end

  def words_per_line!(path) do
    File.stream!(path)
    |> Stream.map(&String.split/1)
    |> Enum.map(&length/1)
  end
end
