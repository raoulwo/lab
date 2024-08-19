defmodule CounterApiTest do
  use ExUnit.Case

  test "use counter through API" do
    pid = Counter.start(0)
    assert 0 == Counter.state(pid)

    Counter.tick(pid)
    Counter.tick(pid)

    assert 2 = Counter.state(pid)
  end
end
