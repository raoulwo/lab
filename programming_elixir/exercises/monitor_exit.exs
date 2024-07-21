defmodule MonitorExit do
  import :timer, only: [sleep: 1]

  def sad_exit do
    receive do
      sender ->
        send(sender, "Hello, sender!")
    end

    exit(:boom)
  end

  def receive_messages do
    receive do
      message ->
        IO.puts(inspect(message))
        receive_messages()
    after
      1000 -> IO.puts("TIMEOUT")
    end
  end

  def run do
    {pid, _monitor_ref} = spawn_monitor(MonitorExit, :sad_exit, [])
    send(pid, self())

    sleep(500)

    receive_messages()
  end
end

MonitorExit.run()
