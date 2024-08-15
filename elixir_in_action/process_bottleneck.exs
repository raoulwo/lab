# NOTE: A single process is a sequential program. If many
# processes send messages to a single program, it may become
# a bottleneck, affecting the overall throughput of the system.
defmodule Server do
  def start do
    spawn(fn -> loop() end)
  end

  def send_msg(server, message) do
    send(server, {self(), message})

    receive do
      {:response, response} -> response
    end
  end

  defp loop do
    receive do
      {caller, message} ->
        Process.sleep(1000)
        send(caller, {:response, msg})
    end

    loop()
  end
end
