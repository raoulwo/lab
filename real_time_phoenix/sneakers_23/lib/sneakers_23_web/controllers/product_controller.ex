defmodule Sneakers23Web.ProductController do
  use Sneakers23Web, :controller

  def home(conn, _params) do
    {:ok, products} = Sneakers23.Inventory.get_complete_products()

    conn
    |> render(:home, products: products)
  end
end
