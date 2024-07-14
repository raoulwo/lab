cast = %{
  buttercup: %{
    actor: {"Robin", "Wright"},
    role: "princess"
  },
  wesley: %{
    actor: {"Cary", "Ewles"},
    role: "farmer"
  }
}

IO.inspect(get_in(cast, [Access.key(:wesley), :actor, Access.elem(1)]))

IO.inspect(
  get_and_update_in(cast, [Access.key(:buttercup), :role], fn role ->
    {role, "queen"}
  end)
)
