# Macros *inject* code into a *context*. This
# context is the scope of the caller's bindings,
# imports and aliases.

# There are **2 contexts** in which a macro executes:
#
# * (1) Context of the macro definition
# * (2) Context of the macro invocation

defmodule Mod do
  defmacro definfo do
    # -> (1)
    # This is executed before the macro is expanded
    # (the AST is transformed in the caller's context).
    IO.puts("in macro's context (#{__MODULE__})")

    quote do
      # -> (2)
      # This is expanded within the caller's context and
      # is *injected* directly into that context.
      IO.puts("in caller's context (#{__MODULE__})")

      def friendly_info do
        IO.puts("""
          My name is #{__MODULE__}  
          My functions are #{inspect(__MODULE__.__info__(:functions))}
        """)
      end
    end
  end
end

defmodule MyModule do
  require Mod
  Mod.definfo()
end
