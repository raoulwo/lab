defmodule MathTest do
  import Assertion

  def run do
    assert 1 == 1
    assert 2 > 1
    assert 1 > 2
    assert 10 * 10 == 100
  end
end
