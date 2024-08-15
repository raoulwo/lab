defmodule TodoServer do
  def start do
    spawn(fn ->
      Process.register(self(), :todo)
      loop(Todo.new())
    end)
  end

  def add_entry(new_entry) do
    send(:todo, {:add_entry, new_entry})
  end

  def entries(date) do
    send(:todo, {:entries, self(), date})

    receive do
      {:todo_entries, entries} -> entries
    after
      5000 -> {:error, :timeout}
    end
  end

  def update_entry(entry_id, updater_fun) do
    send(:todo, {:update_entry, entry_id, updater_fun})
  end

  def delete_entry(entry_id) do
    send(:todo, {:delete_entry, entry_id})
  end

  defp loop(curr_todo) do
    new_todo =
      receive do
        message -> process_message(curr_todo, message)
      end

    loop(new_todo)
  end

  defp process_message(curr_todo, {:add_entry, new_entry}) do
    Todo.add_entry(curr_todo, new_entry)
  end

  defp process_message(curr_todo, {:entries, caller, date}) do
    send(caller, {:todo_entries, Todo.entries(curr_todo, date)})
    curr_todo
  end

  defp process_message(curr_todo, {:update_entry, entry_id, updater_fun}) do
    Todo.update_entry(curr_todo, entry_id, updater_fun)
  end

  defp process_message(curr_todo, {:delete_entry, entry_id}) do
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

defmodule Todo.CsvImporter do
  def import(path) do
    File.stream!(path)
    |> Stream.map(&String.trim_trailing(&1, "\n"))
    |> Stream.map(&String.split(&1, ","))
    |> Stream.map(fn [date, title] -> %{date: Date.from_iso8601!(date), title: title} end)
    |> Todo.new()
  end
end

defimpl Collectable, for: Todo do
  def into(original) do
    {original, &into_callback/2}
  end

  defp into_callback(todo, {:cont, entry}) do
    Todo.add_entry(todo, entry)
  end

  defp into_callback(todo, :done), do: todo
  defp into_callback(_todo, :halt), do: :ok
end
