defmodule Sneakers23.Inventory.Item do
  use Ecto.Schema
  import Ecto.Changeset

  schema "items" do
    field :sku, :string
    field :size, :string
    belongs_to :product, Sneakers23.Inventory.Product

    timestamps()
  end

  @doc false
  def changeset(item, attrs) do
    item
    |> cast(attrs, [:sku, :size, :product_id])
    |> validate_required([:sku, :size, :product_id])
  end
end
