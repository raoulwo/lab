defmodule Sneakers23.Inventory.Inventory do
  defstruct items: %{}, products: %{}, availabilities: %{}

  def new() do
    %__MODULE__{}
  end

  def add_products(state = %{products: products}, to_add) do
    new_products =
      Enum.reduce(to_add, products, fn product, products ->
        Map.put(products, product.id, product)
      end)

    %{state | products: new_products}
  end

  def add_items(state = %{items: items}, to_add) do
    new_items =
      Enum.reduce(to_add, items, fn item, items ->
        Map.put(items, item.id, item)
      end)

    %{state | items: new_items}
  end

  def add_availabilities(state = %{availabilities: availabilities}, to_add) do
    new_availabilities =
      Enum.reduce(to_add, availabilities, fn availability, availabilities ->
        Map.put(availabilities, availability.id, availability)
      end)

    %{state | availabilities: new_availabilities}
  end

  def get_items_for_product(%{items: items}, %{id: product_id}) do
    items
    |> Map.values()
    |> Enum.filter(&(&1.product_id == product_id))
  end

  def get_availability_for_item(%{availabilities: availabilities}, %{id: item_id}) do
    availabilities
    |> Map.values()
    |> Enum.find(&(&1.item_id == item_id))
  end

  def mark_product_released!(state = %{products: products}, id) do
    new_product =
      products
      |> Map.fetch!(id)
      |> Map.put(:released, true)

    new_products = Map.put(products, id, new_product)
    %{state | products: new_products}
  end
end
