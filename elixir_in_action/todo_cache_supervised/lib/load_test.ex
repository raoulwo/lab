defmodule LoadTest do
  @total_processes 1_000_000

  # Start from command line with:
  #   elixir --erl "+P 2000000" -S mix run -e LoadTest.run
  def run do
    {:ok, _} = Todo.Cache.start_link(nil)

    {put_time, _} =
      :timer.tc(fn ->
        Enum.each(1..@total_processes, &Todo.Cache.server_process("todo_#{&1}"))
      end)

    IO.puts("average put time: #{put_time / @total_processes} μs")

    {get_time, _} =
      :timer.tc(fn ->
        Enum.each(1..@total_processes, &Todo.Cache.server_process("todo_#{&1}"))
      end)

    IO.puts("average get time: #{get_time / @total_processes} μs")
  end
end
