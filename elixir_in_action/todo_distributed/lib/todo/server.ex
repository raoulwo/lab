defmodule Todo.Server do
  use GenServer, restart: :temporary

  def start_link(name) do
    GenServer.start_link(__MODULE__, name, name: global_name(name))
  end

  defp global_name(name) do
    {:global, {__MODULE__, name}}
  end

  def whereis(name) do
    # The lookup is performed on a local ETS table, it doesn't
    # load to cross-node chatter
    case :global.whereis_name({__MODULE__, name}) do
      :undefined -> nil
      pid -> pid
    end
  end

  def add_entry(server_pid, new_entry) do
    GenServer.call(server_pid, {:add_entry, new_entry})
  end

  def entries(server_pid, date) do
    GenServer.call(server_pid, {:entries, date})
  end

  def update_entry(server_pid, entry_id, updater_fun) do
    GenServer.cast(server_pid, {:update_entry, entry_id, updater_fun})
  end

  def delete_entry(server_pid, entry_id) do
    GenServer.cast(server_pid, {:delete_entry, entry_id})
  end

  @impl GenServer
  def init(name) do
    # The tuple `{:continue, arg}` splits the initialization into 2 phases:
    #
    # 1. init/1 which blocks the client until it returns
    # 2. handle_continue/2 which is performed after GenServer.start/3
    IO.puts("starting the todo server for #{name}")
    {:ok, {name, nil}, {:continue, :init}}
  end

  @impl GenServer
  def handle_continue(:init, {name, nil}) do
    # Since this operation might take long we extract it into this callback
    # so that the clients aren't blocked
    todo = Todo.Database.get(name) || Todo.List.new()
    {:noreply, {name, todo}, expiry_idle_timeout()}
  end

  @impl GenServer
  def handle_call({:entries, date}, _, {name, curr_todo}) do
    {:reply, Todo.List.entries(curr_todo, date), {name, curr_todo}, expiry_idle_timeout()}
  end

  # The asynchronous Todo.Database.store/2 requests can lead to a bottleneck
  # should the requests come in faster than they can be processed by the db

  @impl GenServer
  def handle_call({:add_entry, new_entry}, _, {name, curr_todo}) do
    new_todo = Todo.List.add_entry(curr_todo, new_entry)
    Todo.Database.store(name, new_todo)
    {:reply, :ok, {name, new_todo}, expiry_idle_timeout()}
  end

  @impl GenServer
  def handle_cast({:update_entry, entry_id, updater_fun}, {name, curr_todo}) do
    new_todo = Todo.List.update_entry(curr_todo, entry_id, updater_fun)
    Todo.Database.store(name, new_todo)
    {:noreply, {name, new_todo}, expiry_idle_timeout()}
  end

  @impl GenServer
  def handle_cast({:delete_entry, entry_id}, {name, curr_todo}) do
    new_todo = Todo.List.delete_entry(curr_todo, entry_id)
    Todo.Database.store(name, new_todo)
    {:noreply, {name, new_todo}, expiry_idle_timeout()}
  end

  @impl GenServer
  def handle_info(:timeout, {name, todo}) do
    IO.puts("stopping the todo server for #{name}")
    {:stop, :normal, {name, todo}}
  end

  defp expiry_idle_timeout, do: Application.fetch_env!(:todo, :todo_server_expiry)
end
