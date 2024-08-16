defmodule Todo.Database do
  use GenServer

  @workers 3

  @db_folder "./persist"

  def start_link do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
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
        {:ok, worker} = Todo.DatabaseWorker.start_link(@db_folder)
        {index, worker}
      end

    {:ok, workers}
  end

  @impl GenServer
  def handle_call({:choose_worker, key}, _, workers) do
    {:reply, Map.get(workers, :erlang.phash2(key, 3)), workers}
  end
end
