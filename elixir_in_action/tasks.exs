run_query = fn query_def ->
  Process.sleep(2000)
  "#{query_def} result"
end

1..5
|> Enum.map(&Task.async(fn -> run_query.("query (#{&1})") end))
# `Task.await/1` links the task process to the starting process
|> Enum.map(&Task.await/1)
|> Enum.each(&IO.puts/1)
