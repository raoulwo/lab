# Using the macros `quote` and `unquote` together makes
# it possible to build ASTs without transforming the
# 3-tuples they're made of manually.
defmodule ControlFlow do
  # Macros take in an AST representation and then
  # return an AST representation.
  defmacro unless(expression, do: block) do
    # The `quote` block will be used to return an AST
    # that is then expanded within the caller's context
    # at compile time. This process is called *macro
    # expansion*. Using `unquote` inside a `quote` block
    # we can inject values into the AST representation.
    # You can think of using `unquote` as interpolating
    # values into a string.
    #
    # `quote`: value -> AST
    # `unquote`: AST -> value
    quote do
      if !unquote(expression), do: unquote(block)
    end
  end
end
