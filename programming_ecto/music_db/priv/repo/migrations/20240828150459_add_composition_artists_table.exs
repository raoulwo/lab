defmodule MusicDB.Repo.Migrations.AddCompositionArtistsTable do
  use Ecto.Migration
  import Ecto.Query
  alias MusicDB.Repo

  def up do
    create table("compositions_artists") do
      add :composition_id, references("compositions"), null: false
      add :artist_id, references("artists"), null: false
      add :role, :string, null: false
    end

    create index("compositions_artists", :composition_id)
    create index("compositions_artists", :artist_id)

    # NOTE: The book says this is required else the table can't be created
    # since all operations are sent to the db at once. For me it instantly
    # worker however.

    # flush()

    from("compositions", select: [:id, :artist_id])
    |> Repo.all()
    |> Enum.each(fn row ->
      Repo.insert_all("compositions_artists", [
        [composition_id: row.id, artist_id: row.artist_id, role: "composer"]
      ])
    end)

    alter table("compositions") do
      remove :artist_id
    end
  end

  def down do
    alter table("compositions") do
      add :artist_id, references("artists")
    end

    from("compositions_artists", where: [role: "composer"],
      select: [:composition_id, :artist_id])
    |> Repo.all()
    |> Enum.each(fn row ->
      Repo.update_all(
        from("compositions", where: [id: ^row.composition_id]),
        set: [artist_id: row.artist_id]
      )
    end)

    drop table("compositions_artists")
  end
end
