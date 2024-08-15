run_query = fn query_def ->
  Process.sleep(2000)
  "#{query_def} result"
end

async_query = fn query_def ->
  pid = self()

  spawn(fn ->
    query_result = run_query.(query_def)
    send(pid, {:query_result, query_result})
  end)
end

get_result = fn ->
  receive do
    {:query_result, query_result} -> IO.inspect(query_result)
  end
end

1..50
|> Enum.map(&"query (#{&1})")
|> Enum.map(&async_query.(&1))
|> Enum.map(fn _ -> get_result.() end)
