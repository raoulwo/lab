# The `use` macro is an API for module extension. By writing
# `use SomeModule` the macro `SomeModule.__using__/1` is called.
# It's such a common API for module extension that it's used
# extensively in many Elixir libraries/frameworks.
defmodule Assertion do
  defmacro __using__(_options) do
    quote do
      import unquote(__MODULE__)

      def run do
        IO.puts("Running the tests...")
      end
    end
  end
end

# NOTE: The `use` macro does code injection into a module just
# like the custom `extend` function in `module_extension_custom.exs`.
# It's just a little bit more convenient.
defmodule MathTest do
  use Assertion
end
