defmodule Ok do
  def ok!(param) do
    case param do
      {:ok, data} -> data
      _ -> raise "Expected tuple of form '{:ok, data}'."
    end
  end
end
