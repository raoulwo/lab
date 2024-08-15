defmodule ServerProcess do
  def start(callback_module) do
    spawn(fn ->
      initial_state = callback_module.init()
      loop(callback_module, initial_state)
    end)
  end

  def call(server_pid, request) do
    send(server_pid, {:call, request, self()})

    receive do
      {:response, response} -> response
    end
  end

  def cast(server_pid, request) do
    send(server_pid, {:cast, request})
  end

  defp loop(callback_module, curr_state) do
    receive do
      {:call, request, caller} ->
        {response, new_state} = callback_module.handle_call(request, curr_state)
        send(caller, {:response, response})
        loop(callback_module, new_state)

      {:cast, request} ->
        new_state = callback_module.handle_cast(request, curr_state)
        loop(callback_module, new_state)
    end
  end
end

defmodule TodoServer do
  def start do
    ServerProcess.start(__MODULE__)
  end

  def add_entry(server_pid, new_entry) do
    ServerProcess.cast(server_pid, {:add_entry, new_entry})
  end

  def entries(server_pid, date) do
    ServerProcess.call(server_pid, {:entries, date})
  end

  def update_entry(server_pid, entry_id, updater_fun) do
    ServerProcess.cast(server_pid, {:update_entry, entry_id, updater_fun})
  end

  def delete_entry(server_pid, entry_id) do
    ServerProcess.cast(server_pid, {:delete_entry, entry_id})
  end

  def init do
    Todo.new()
  end

  def handle_call({:entries, date}, curr_todo) do
    {Todo.entries(curr_todo, date), curr_todo}
  end

  def handle_cast({:add_entry, new_entry}, curr_todo) do
    Todo.add_entry(curr_todo, new_entry)
  end

  def handle_cast({:update_entry, entry_id, updater_fun}, curr_todo) do
    Todo.update_entry(curr_todo, entry_id, updater_fun)
  end

  def handle_cast({:delete_entry, entry_id}, curr_todo) do
    Todo.delete_entry(curr_todo, entry_id)
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
