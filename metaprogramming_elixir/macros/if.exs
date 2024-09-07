defmodule ControlFlow do
  defmacro my_if(expr, do: if_block) do
    quote do
      case unquote(expr) do
        x when x in [nil, false] ->
          nil

        _ ->
          unquote(if_block)
      end
    end
  end

  defmacro my_if(expr, do: if_block, else: else_block) do
    quote do
      case unquote(expr) do
        x when x in [nil, false] ->
          unquote(else_block)

        _ ->
          unquote(if_block)
      end
    end
  end
end
