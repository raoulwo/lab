# NOTE: The module attribute `@before_compile` can be used
# to notify the compiler that an extra step is required
# just before compilation is finished. This attribute
# accepts a *module* as an argument. The module passed
# to `@before_compile` needs to define a `__before_compile__/1`
# macro which is invoked just before compilation to
# perform code gen.

# In our case, this will allow for
# `run/0` to be generated **after** all `test` macros
# have expanded so that it has access to the list of
# test blocks within `@tests`.
defmodule Assertion do
  defmacro __using__(_options) do
    quote do
      import unquote(__MODULE__)
      Module.register_attribute(__MODULE__, :tests, accumulate: true)

      # Here, we're specifying that the module `Assertion` defines
      # a `__before_compile__/1` macro that is called before compilation
      # is finished, however after the other code generation has taken
      # place.
      @before_compile unquote(__MODULE__)
    end
  end

  defmacro __before_compile__(_env) do
    quote do
      def run do
        IO.puts("Running the tests (#{inspect(@tests)})")
      end
    end
  end

  defmacro test(description, do: test_block) do
    test_func = String.to_atom(description)

    quote do
      @tests {unquote(test_func), unquote(description)}

      def unquote(test_func)(), do: unquote(test_block)
    end
  end

  defmacro assert({operator, _, [lhs, rhs]}) do
    quote bind_quoted: [operator: operator, lhs: lhs, rhs: rhs] do
      Assertion.Test.assert(operator, lhs, rhs)
    end
  end

  defmodule Test do
    def assert(:==, lhs, rhs) when lhs == rhs do
      IO.write(".")
    end

    def assert(:==, lhs, rhs) do
      IO.puts("""
      FAILURE:
        Expected:       #{lhs}
        to be equal to: #{rhs}
      """)
    end

    def assert(:>, lhs, rhs) when lhs > rhs do
      IO.write(".")
    end

    def assert(:>, lhs, rhs) do
      IO.puts("""
      FAILURE:
        Expected:         #{lhs}
        to be greater to: #{rhs}
      """)
    end
  end
end
