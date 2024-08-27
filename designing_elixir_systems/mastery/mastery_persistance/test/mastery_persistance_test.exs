defmodule MasteryPersistanceTest do
  use ExUnit.Case

  alias MasteryPersistance.{Repo, Response}

  setup do
    :ok = Ecto.Adapters.SQL.Sandbox.checkout(Repo)

    response = %{
      quiz_title: :simple_addition,
      template_name: :single_digit_addition,
      to: "3 + 4",
      email: "student@example.com",
      answer: "7",
      correct: true,
      timestamp: DateTime.utc_now()
    }

    {:ok, %{response: response}}
  end

  test "responses are recorded", %{response: response} do
    assert Repo.aggregate(Response, :count, :id) == 0
    assert :ok = MasteryPersistance.record_response(response)
    assert Repo.all(Response) |> Enum.map(fn res -> res.email end) == [response.email]
  end

  test "a function can be run in the saving transaction", %{response: response} do
    assert response.answer == MasteryPersistance.record_response(response, fn res -> res.answer end)
  end

  test "an error in the function rolls back the save", %{response: response} do
    assert Repo.aggregate(Response, :count, :id) == 0
    assert_raise RuntimeError, fn ->
      MasteryPersistance.record_response(response, fn _res -> raise "boom" end)
    end
    assert Repo.aggregate(Response, :count, :id) == 0
  end

  test "simple reporting", %{response: response} do
    MasteryPersistance.record_response(response)
    MasteryPersistance.record_response(response)

    response
    |> Map.put(:email, "other_#{response.email}")
    |> MasteryPersistance.record_response()

    assert MasteryPersistance.report(response.quiz_title) == %{
      response.email => 2,
      "other_#{response.email}" => 1
    }
  end
end
