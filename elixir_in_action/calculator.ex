defmodule Calculator do
  def start do
    spawn(fn ->
      loop(0)
    end)
  end

  def value(server_pid) do
    send(server_pid, {:value, self()})

    receive do
      {:response, value} -> value
    end
  end

  def add(server_pid, value), do: send(server_pid, {:add, value})
  def sub(server_pid, value), do: send(server_pid, {:sub, value})
  def mul(server_pid, value), do: send(server_pid, {:mul, value})
  def div(server_pid, value), do: send(server_pid, {:div, value})

  defp loop(curr_value) do
    new_value =
      receive do
        message -> process_message(curr_value, message)
      end

    loop(new_value)
  end

  defp process_message(curr_value, {:value, caller}) do
    send(caller, {:response, curr_value})
  end

  defp process_message(curr_value, {:add, value}) do
    curr_value + value
  end

  defp process_message(curr_value, {:sub, value}) do
    curr_value - value
  end

  defp process_message(curr_value, {:mul, value}) do
    curr_value * value
  end

  defp process_message(curr_value, {:div, value}) do
    curr_value / value
  end

  defp process_message(curr_value, message) do
    IO.puts("invalid request: #{inspect(message)}")
    curr_value
  end
end
