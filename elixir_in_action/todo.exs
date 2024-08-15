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
  defp into_callback(todo, :halt), do: :ok
end
