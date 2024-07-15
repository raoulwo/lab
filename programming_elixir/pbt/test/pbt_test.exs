defmodule PbtTest do
  use ExUnit.Case
  use ExUnitProperties
  doctest Pbt

  describe "stats on lists of ints" do
    setup do
      [
        list: [1, 3, 5, 7, 9, 11],
        sum: 36,
        count: 6
      ]
    end

    test "calculates sum", fixture do
      assert Pbt.sum(fixture.list) == fixture.sum
    end

    test "calculates length", fixture do
      assert Pbt.count(fixture.list) == fixture.count
    end

    test "calculates average", fixture do
      assert Pbt.average(fixture.list) == fixture.sum / fixture.count
    end

    property "count not negative" do
      check all(l <- list_of(integer())) do
        assert Pbt.count(l) >= 0
      end
    end

    property "single element lists are their own sum" do
      check all(number <- integer()) do
        assert Pbt.sum([number]) == number
      end
    end

    property "sum equals average times count (nonempty)" do
      check all(l <- list_of(integer()) |> nonempty) do
        assert_in_delta(Pbt.sum(l), Pbt.count(l) * Pbt.average(l), 1.0e-6)
      end
    end
  end
end
