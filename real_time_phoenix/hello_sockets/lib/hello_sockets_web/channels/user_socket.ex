defmodule HelloSocketsWeb.UserSocket do
  use Phoenix.Socket

  channel "ping", HelloSocketsWeb.PingChannel
  channel "ping:*", HelloSocketsWeb.PingChannel
  channel "wild:*", HelloSocketsWeb.WildcardChannel
  channel "dupe", HelloSocketsWeb.DedupeChannel

  @impl Phoenix.Socket
  def connect(_params, socket, _connect_info) do
    {:ok, socket}
  end

  @impl Phoenix.Socket
  def id(_socket), do: nil
end
