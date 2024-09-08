# INFO: Elixir supports so called *unquote fragments* which make it possible
# to dynamically define functions. In this example we use them to define multiple
# clauses of the `exts_from_type` and `type_from_ext` functions, however in the
# simple testing framework we also used them to define function names on the fly.

defmodule Mime do
  # When defining an external resource you can register the `@external_resource`
  # attribute. By doing so, the Elixir module will be recompiled by mix should 
  # the specified compile-time resources change.
  @external_resource mimes_path = Path.join([__DIR__, "mimes.txt"])

  for line <- File.stream!(mimes_path, :line) do
    [type, rest] = line |> String.split("\t") |> Enum.map(&String.trim(&1))
    extensions = String.split(rest, ~r/,\s?/)

    # INFO: It's possible to use unquote fragments even outside of `quote` blocks!

    # Here, we're defining a function that maps from MIME type to file extension(s).
    def exts_from_type(unquote(type)), do: unquote(extensions)
    # Here, we're defining a function that maps from file extension to MIME type.
    def type_from_ext(ext) when ext in unquote(extensions), do: unquote(type)
  end

  # Catch-all definitions for `exts_from_type` and `type_from_ext`
  def exts_from_type(_type), do: []
  def type_from_ext(_ext), do: nil
  def valid_type?(type), do: exts_from_type(type) |> Enum.any?()
end
