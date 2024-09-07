defmodule Assertion do
  # {:==, [context: Elixir, import: Kernel], [1, 1]}
  defmacro assert({operator, _, [lhs, rhs]}) do
    # `bind_quoted` assures that its bindings are unquoted inside
    # the `quote` block and then bound to a variable **exactly** once.
    # This prevents us from accidentally evaluating an expression
    # multiple times.
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
