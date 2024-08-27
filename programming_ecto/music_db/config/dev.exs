# ---
# Excerpted from "Programming Ecto",
# published by The Pragmatic Bookshelf.
# Copyrights apply to this code. It may not be used to create training material,
# courses, books, articles, and the like. Contact us if you are in doubt.
# We make no guarantees that this code is fit for any purpose.
# Visit https://pragprog.com/titles/wmecto for more book information.
# ---
import Config

config :music_db, MusicDB.Repo,
  database: "music_db",
  username: "raoul",
  password: "postgres",
  hostname: "localhost"
