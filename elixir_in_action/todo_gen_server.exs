defmodule TodoServer do
  use GenServer

  def start do
    GenServer.start(__MODULE__, Todo.new(), name: __MODULE__)
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
    {:reply, Todo.entries(curr_todo, date), curr_todo}
  end

  @impl GenServer
  def handle_cast({:add_entry, new_entry}, curr_todo) do
    {:noreply, Todo.add_entry(curr_todo, new_entry)}
  end

  @impl GenServer
  def handle_cast({:update_entry, entry_id, updater_fun}, curr_todo) do
    {:noreply, Todo.update_entry(curr_todo, entry_id, updater_fun)}
  end

  @impl GenServer
  def handle_cast({:delete_entry, entry_id}, curr_todo) do
    {:noreply, Todo.delete_entry(curr_todo, entry_id)}
  end
end

defmodule Todo do
  defstruct next_id: 1, entries: %{}

  def new(entries \\ []) do
    Enum.reduce(entries, %Todo{}, &Todo.add_entry(&2, &1))
  end

  def add_entry(todo, entry) do
    entry = Map.put(entry, :id, todo.next_id)

    new_entries = Map.put(todo.entries, todo.next_id, entry)

    %Todo{todo | next_id: todo.next_id + 1, entries: new_entries}
  end

  def entries(todo, date) do
    todo.entries
    |> Map.values()
    |> Enum.filter(fn entry -> date == entry.date end)
  end

  def update_entry(todo, entry_id, updater_fun) do
    case Map.fetch(todo.entries, entry_id) do
      :error ->
        todo

      {:ok, old_entry} ->
        new_entry = updater_fun.(old_entry)
        new_entries = Map.put(todo.entries, entry_id, new_entry)
        %Todo{todo | entries: new_entries}
    end
  end

  def delete_entry(todo, entry_id) do
    new_entries = Map.delete(todo.entries, entry_id)
    %Todo{todo | entries: new_entries}
  end
end
