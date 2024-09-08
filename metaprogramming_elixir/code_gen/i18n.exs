defmodule I18n do
  use Translator

  locale("en",
    flash: [
      hello: "Hello %{first} %{last}!",
      bye: "Bye, %{name}!"
    ],
    users: [
      title: "Users"
    ]
  )

  locale("de",
    flash: [
      hello: "Servus %{first} %{last}!",
      bye: "Auf Wiedersehen, %{name}!"
    ],
    users: [
      title: "Benutzer"
    ]
  )
end
