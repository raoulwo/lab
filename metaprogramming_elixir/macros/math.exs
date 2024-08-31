# Macro Rules:
#
# 1. Don't write macros
# 2. Use macros gratuitously

# All elixir code is represented in the AST as 3-tuples:
#
# 1. atom describing a function call or another tuple
# 2. metadata about the expression
# 3. list of arguments for the function call
#
# Entire elixir programs can be represented with a tree
# consisting of these 3-tuples.

defmodule Math do
  # {:+, [context: Elixir, import: Kernel], [5, 2]}
  defmacro say({:+, _, [lhs, rhs]}) do
    quote do
      lhs = unquote(lhs)
      rhs = unquote(rhs)
      result = lhs + rhs
      IO.puts("#{lhs} plus #{rhs} is #{result}")
      result
    end
  end

  # {:*, [context: Elixir, import: Kernel], [8, 3]}
  defmacro say({:*, _, [lhs, rhs]}) do
    quote do
      lhs = unquote(lhs)
      rhs = unquote(rhs)
      result = lhs * rhs
      IO.puts("#{lhs} times #{rhs} is #{result}")
    end
  end
end
