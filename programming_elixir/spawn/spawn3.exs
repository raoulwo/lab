defmodule Spawn3 do
  def greet do
    receive do
      {sender, msg} ->
        send(sender, {:ok, "Hello, #{msg}!"})
    end
  end
end

pid = spawn(Spawn3, :greet, [])

send(pid, {self(), "concurrent world"})

receive do
  {:ok, message} -> IO.puts(message)
end

receive do
  {:ok, message} -> IO.puts(message)
after
  500 -> IO.puts("The greeter ate itself...")
end
