cast = [
  %{
    character: "Buttercup",
    actor: {"robin", "Wright"},
    role: "princess"
  },
  %{
    character: "Wesley",
    actor: {"cary", "Ewles"},
    role: "farmer"
  }
]

IO.inspect(get_in(cast, [Access.all(), :actor, Access.elem(1)]))

IO.inspect(
  get_and_update_in(cast, [Access.all(), :actor, Access.elem(0)], fn first ->
    {first, String.capitalize(first)}
  end)
)
