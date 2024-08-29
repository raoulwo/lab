defmodule HelloSocketsWeb.PingChannel do
  use Phoenix.Channel

  intercept ["request_ping"]

  @impl Phoenix.Channel
  def join(_topic, _payload, socket) do
    {:ok, socket}
  end

  @impl Phoenix.Channel
  def handle_in("ping", %{"ack_phrase" => ack_phrase}, socket) do
    {:reply, {:ok, %{ping: ack_phrase}}, socket}
  end

  @impl Phoenix.Channel
  def handle_in("ping", _payload, socket) do
    {:reply, {:ok, %{ping: "pong"}}, socket}
  end

  @impl Phoenix.Channel
  def handle_in("ping:" <> phrase, _payload, socket) do
    {:reply, {:ok, %{ping: phrase}}, socket}
  end

  @impl Phoenix.Channel
  def handle_in("pong", _payload, socket) do
    {:noreply, socket}
  end

  @impl Phoenix.Channel
  def handle_in("ding", _payload, socket) do
    {:stop, :shutdown, {:ok, %{msg: "shutting down"}}, socket}
  end

  @impl Phoenix.Channel
  def handle_in("param_ping", %{"error" => true}, socket) do
    {:reply, {:error, %{reason: "You asked for this!"}}, socket}
  end

  @impl Phoenix.Channel
  def handle_in("param_ping", payload, socket) do
    {:reply, {:ok, payload}, socket}
  end

  @impl Phoenix.Channel
  def handle_out("request_ping", payload, socket) do
    push(socket, "send_ping", Map.put(payload, "from_node", Node.self()))
    {:noreply, socket}
  end
end
