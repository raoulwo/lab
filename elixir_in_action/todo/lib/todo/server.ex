defmodule Todo.Server do
  use GenServer

  def start do
    GenServer.start(__MODULE__, Todo.List.new(), name: __MODULE__)
  end

  def add_entry(new_entry) do
    GenServer.cast(__MODULE__, {:add_entry, new_entry})
  end

  def entries(date) do
    GenServer.call(__MODULE__, {:entries, date})
  end

  def update_entry(entry_id, updater_fun) do
    GenServer.cast(__MODULE__, {:update_entry, entry_id, updater_fun})
  end

  def delete_entry(entry_id) do
    GenServer.cast(__MODULE__, {:delete_entry, entry_id})
  end

  @impl GenServer
  def init(initial_state) do
    {:ok, initial_state}
  end

  @impl GenServer
  def handle_call({:entries, date}, _, curr_todo) do
    {:reply, Todo.List.entries(curr_todo, date), curr_todo}
  end

  @impl GenServer
  def handle_cast({:add_entry, new_entry}, curr_todo) do
    {:noreply, Todo.List.add_entry(curr_todo, new_entry)}
  end

  @impl GenServer
  def handle_cast({:update_entry, entry_id, updater_fun}, curr_todo) do
    {:noreply, Todo.List.update_entry(curr_todo, entry_id, updater_fun)}
  end

  @impl GenServer
  def handle_cast({:delete_entry, entry_id}, curr_todo) do
    {:noreply, Todo.List.delete_entry(curr_todo, entry_id)}
  end
end
