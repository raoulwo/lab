defmodule Scope do
  defmacro update_local(var) do
    local = "local macro definition value"

    result =
      quote do
        local = unquote(var)
        IO.puts("Inside macro, local = #{local}")
      end

    IO.puts("Inside macro definition, local = #{local}")

    result
  end
end

defmodule Test do
  require Scope

  local = "Outside of macro"
  Scope.update_local("Calling the macro")
  IO.puts("After calling macro, local = #{local}")
end
