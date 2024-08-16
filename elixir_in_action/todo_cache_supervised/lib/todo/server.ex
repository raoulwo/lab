defmodule Todo.Server do
  use GenServer

  def start_link(name) do
    GenServer.start_link(__MODULE__, name)
  end

  def add_entry(server_pid, new_entry) do
    GenServer.cast(server_pid, {:add_entry, new_entry})
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
    {:ok, {name, nil}, {:continue, :init}}
  end

  @impl GenServer
  def handle_continue(:init, {name, nil}) do
    # Since this operation might take long we extract it into this callback
    # so that the clients aren't blocked
    todo = Todo.Database.get(name) || Todo.List.new()
    {:noreply, {name, todo}}
  end

  @impl GenServer
  def handle_call({:entries, date}, _, {_name, curr_todo}) do
    {:reply, Todo.List.entries(curr_todo, date), curr_todo}
  end

  # The asynchronous Todo.Database.store/2 requests can lead to a bottleneck
  # should the requests come in faster than they can be processed by the db

  @impl GenServer
  def handle_cast({:add_entry, new_entry}, {name, curr_todo}) do
    new_todo = Todo.List.add_entry(curr_todo, new_entry)
    Todo.Database.store(name, new_todo)
    {:noreply, {name, new_todo}}
  end

  @impl GenServer
  def handle_cast({:update_entry, entry_id, updater_fun}, {name, curr_todo}) do
    new_todo = Todo.List.update_entry(curr_todo, entry_id, updater_fun)
    Todo.Database.store(name, new_todo)
    {:noreply, {name, new_todo}}
  end

  @impl GenServer
  def handle_cast({:delete_entry, entry_id}, {name, curr_todo}) do
    new_todo = Todo.List.delete_entry(curr_todo, entry_id)
    Todo.Database.store(name, new_todo)
    {:noreply, {name, new_todo}}
  end
end
