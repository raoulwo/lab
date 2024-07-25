defmodule ColorSigil do
  @color_map [
    rgb: [red: 0xFF0000, green: 0x00FF00, blue: 0x0000FF],
    hsb: [red: {0, 100, 100}, green: {120, 100, 100}, blue: {240, 100, 100}]
  ]

  def sigil_k(color_name, []), do: _k(color_name, :rgb)
  def sigil_k(color_name, ~c"r"), do: _k(color_name, :rgb)
  def sigil_k(color_name, ~c"h"), do: _k(color_name, :hsb)

  defp _k(color_name, color_space) do
    @color_map[color_space][String.to_atom(color_name)]
  end
end

defmodule Example do
  import ColorSigil

  def rgb, do: IO.inspect(~k{red})
  def hsb, do: IO.inspect(~k{red}h)
end

Example.rgb()
Example.hsb()
