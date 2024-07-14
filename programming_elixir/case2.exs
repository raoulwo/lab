raoul = %{ name: "Raoul", age: 23 }
case raoul do
  person = %{age: age} when is_number(age) and age >= 21 ->
    IO.puts("You're cleared to enter the Foo Bar, #{person.name}.")
  _ ->
    IO.puts("Sorry, no admission.")
end
