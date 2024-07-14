cast = [
  %{
    character: "Buttercup",
    actor: %{
      first: "Robin",
      last: "Wright"
    },
    role: "princess"
  },
  %{
    character: "Wesley",
    actor: %{
      first: "Cary",
      last: "Ewles"
    },
    role: "farmer"
  }
]

IO.inspect(get_in(cast, [Access.all(), :character]))

IO.inspect(get_in(cast, [Access.at(1), :role]))

IO.inspect(
  get_and_update_in(cast, [Access.all(), :role], fn role ->
    {role, String.capitalize(role)}
  end)
)
