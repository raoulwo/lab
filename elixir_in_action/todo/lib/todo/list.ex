defmodule Todo.List do
  defstruct next_id: 1, entries: %{}

  def new(entries \\ []) do
    Enum.reduce(entries, %Todo.List{}, &Todo.List.add_entry(&2, &1))
  end

  def add_entry(todo, entry) do
    entry = Map.put(entry, :id, todo.next_id)

    new_entries = Map.put(todo.entries, todo.next_id, entry)

    %Todo.List{todo | next_id: todo.next_id + 1, entries: new_entries}
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
        %Todo.List{todo | entries: new_entries}
    end
  end

  def delete_entry(todo, entry_id) do
    new_entries = Map.delete(todo.entries, entry_id)
    %Todo.List{todo | entries: new_entries}
  end
end
