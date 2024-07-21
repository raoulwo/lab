defmodule LinkException do
  import :timer, only: [sleep: 1]

  def sad_exit do
    receive do
      sender ->
        send(sender, "Hello, sender!")
    end

    raise("boom")
  end

  def receive_messages do
    receive do
      message -> IO.puts(message)
    end

    receive_messages()
  end

  def run do
    pid = spawn_link(LinkException, :sad_exit, [])
    send(pid, self())

    sleep(500)

    receive_messages()
  end
end

LinkException.run()
