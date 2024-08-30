defmodule HelloSocketsWeb.PingChannelTest do
  use HelloSocketsWeb.ChannelCase
  alias HelloSocketsWeb.UserSocket

  describe "join/3 success" do
    test "ok when joining via ping topic" do
      {:ok, _, %Phoenix.Socket{}} =
        socket(UserSocket, nil, %{})
        |> subscribe_and_join("ping", %{})
    end
  end

  describe "handle_in ping" do
    test "a pong response is provided" do
      assert {:ok, _, socket} =
               socket(UserSocket, nil, %{})
               |> subscribe_and_join("ping", %{})

      ref = push(socket, "ping", %{})
      reply = %{ping: "pong"}
      assert_reply ref, :ok, ^reply
    end
  end
end
