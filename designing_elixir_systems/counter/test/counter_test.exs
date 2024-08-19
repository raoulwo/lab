defmodule CounterTest do
  use ExUnit.Case

  test "inc increments an integer value" do
    assert 2 == Counter.Core.inc(1)
  end
end
