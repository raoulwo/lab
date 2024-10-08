defmodule HelloSocketsWeb.WildcardChannelTest do
  use HelloSocketsWeb.ChannelCase
  import ExUnit.CaptureLog
  alias HelloSocketsWeb.UserSocket

  describe "join/3 success" do
    test "ok when numbers in format a:b where b = 2*a" do
      assert {:ok, _, %Phoenix.Socket{}} =
               socket(UserSocket, nil, %{})
               |> subscribe_and_join("wild:2:4", %{})

      assert {:ok, _, %Phoenix.Socket{}} =
               socket(UserSocket, nil, %{})
               |> subscribe_and_join("wild:100:200", %{})
    end
  end

  describe "join/3 error" do
    test "error when b != 2*a" do
      assert socket(UserSocket, nil, %{})
             |> subscribe_and_join("wild:1:3", %{}) == {:error, %{}}
    end

    test "error when 3 numbers are provided" do
      assert socket(UserSocket, nil, %{})
             |> subscribe_and_join("wild:1:2:3", %{}) == {:error, %{}}
    end
  end

  describe "join/3 error causing crash" do
    test "error with an invalid format topic" do
      assert capture_log(fn ->
               socket(UserSocket, nil, %{})
               |> subscribe_and_join("wild:invalid", %{})
             end) =~ "[error] an exception was raised"
    end
  end
end
