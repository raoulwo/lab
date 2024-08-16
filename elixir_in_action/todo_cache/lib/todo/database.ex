defmodule Todo.Database do
  use GenServer

  @workers 3

  @db_folder "./persist"

  def start do
    GenServer.start(__MODULE__, nil, name: __MODULE__)
  end

  def store(key, data) do
    key
    |> choose_worker()
    |> Todo.DatabaseWorker.store(key, data)
  end

  def get(key) do
    key
    |> choose_worker()
    |> Todo.DatabaseWorker.get(key)
  end

  def choose_worker(key) do
    GenServer.call(__MODULE__, {:choose_worker, key})
  end

  @impl GenServer
  def init(_) do
    File.mkdir_p!(@db_folder)

    workers =
      for index <- 0..(@workers - 1), into: %{} do
        {:ok, worker} = Todo.DatabaseWorker.start(@db_folder)
        {index, worker}
      end

    {:ok, workers}
  end

  @impl GenServer
  def handle_call({:choose_worker, key}, _, workers) do
    {:reply, Map.get(workers, :erlang.phash2(key, 3)), workers}
  end
end

defmodule Todo.DatabaseWorker do
  use GenServer

  def start(db_folder) do
    GenServer.start(__MODULE__, db_folder)
  end

  def store(server_pid, key, data) do
    GenServer.cast(server_pid, {:store, key, data})
  end

  def get(server_pid, key) do
    GenServer.call(server_pid, {:get, key})
  end

  @impl GenServer
  def init(db_folder) do
    {:ok, db_folder}
  end

  @impl GenServer
  def handle_cast({:store, key, data}, db_folder) do
    IO.inspect("worker #{inspect(self())}: store #{key}")

    key
    |> file_name(db_folder)
    |> File.write!(:erlang.term_to_binary(data))

    {:noreply, db_folder}
  end

  @impl GenServer
  def handle_call({:get, key}, _, db_folder) do
    IO.inspect("worker #{inspect(self())}: get #{key}")

    data =
      case File.read(file_name(key, db_folder)) do
        {:ok, contents} -> :erlang.binary_to_term(contents)
        _ -> nil
      end

    {:reply, data, db_folder}
  end

  defp file_name(key, db_folder) do
    Path.join(db_folder, to_string(key))
  end
end
