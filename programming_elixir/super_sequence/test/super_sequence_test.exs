defmodule SuperSequenceTest do
  use ExUnit.Case
  doctest SuperSequence

  test "greets the world" do
    assert SuperSequence.hello() == :world
  end
end
