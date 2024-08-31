# There exists a concept in elixir called *macro hygiene*.
# Hygiene describes that variables, imports, aliases from a
# macro **don't** leak into the caller's own definitions.

# Sometimes it is a necessary evil to implicitly access the
# caller's scope in an *unhygienic* way. When that's the case
# we need to take extra care.

# Elixir requires you to explicitly define variables inside
# macros that should bind to values in their caller's context.
# You can do this using the `var!` macro to mark a variable
# so that it shouldn't be *hygienized*.

defmodule Setter do
  defmacro hygienic_bind_name(string) do
    # Because macros are hygienic by default, they can't
    # access and alter the caller's scope.
    quote do
      name = unquote(string)
    end
  end

  defmacro unhygienic_bind_name(string) do
    # In order to produce an AST that has access to the
    # caller's bindings we use the macro `var!`. Now the
    # variable `name` isn't hygienized anymore.
    quote do
      var!(name) = unquote(string)
    end
  end
end

# NOTE: Selectively using unhygienic macros can be required
# in some advanced use cases, you should be very careful about
# doing so however.
