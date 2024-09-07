# NOTE: Module attributes make it possible to store data in 
# a module at compile time. They're often used in places where
# other languages would constants instead. By specifying the
# `accumulate: true` option when registering an attribute, we
# can keep an appended list of registrations during the compile
# phase. After the module is compiled, the attribute will contain
# a list of all registrations to it at compile-time.

# In our small testing framework we'll keep a list of all registered
# test blocks.
defmodule Assertion do
  defmacro __using__(_options) do
    quote do
      import unquote(__MODULE__)
      # Here, we're programmatically registering the `@tests`
      # attribute into the module which will include the testing
      # framework using `use Assertion`.

      # NOTE: Inside a `quote` block `unquote(__MODULE__)` will expand
      # to current module where we're defining the macro in (`Assertion`
      # in this case). `__MODULE__` on the other hand will expand into
      # the module name where the AST expressions are injected into.
      Module.register_attribute(__MODULE__, :tests, accumulate: true)

      # WARNING: This function call will always output an empty list of
      # test blocks since this function will expand into target module
      # before any `test` macros. To make this work, we need to delay
      # this macro expansion after other code generation has taken place.
      def run do
        IO.puts("Running the tests (#{inspect(@tests)})")
      end
    end
  end

  # Each `test` block will accept a description followed by a keyword
  # list for the `do/end` block of test code.
  defmacro test(description, do: test_block) do
    test_func = String.to_atom(description)

    quote do
      # Here, we're accumulating the name of the test function and
      # the description of the test block in the `@tests` attribute.
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
