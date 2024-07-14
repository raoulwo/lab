case File.open("case.exs") do
  {:ok, file} -> IO.puts("First line: #{IO.read(file, :line)}")
  {:error, reason} -> IO.puts("Failed to open file: #{reason}")
end

case File.open("case2.exs") do
  {:ok, file} -> IO.puts("First line: #{IO.read(file, :line)}")
  {:error, reason} -> IO.puts("Failed to open file: #{reason}")
end
