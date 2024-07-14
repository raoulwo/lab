defmodule Sales do
  def read_csv(path) do
    File.open!(path)
    |> IO.stream(:line)
    |> Enum.drop(1)
    |> Enum.to_list()
  end

  def parse_csv(list) do
    Enum.map(list, &_parse_row/1)
  end

  defp _parse_row(row) do
    [id, ship_to, net_amount] = row |> String.trim() |> String.split(",")

    [
      id: String.to_integer(id),
      ship_to: ship_to |> String.trim_leading(":") |> String.to_atom(),
      net_amount: String.to_float(net_amount)
    ]
  end

  def sales_tax(list) do
    tax_rates = %{NC: 0.075, TX: 0.08}

    Enum.map(
      list,
      fn
        row = [_, {:ship_to, ship_to}, {:net_amount, net_amount}] ->
          if ship_to in Map.keys(tax_rates) do
            row ++ [total_amount: net_amount + net_amount * tax_rates[ship_to]]
          else
            row ++ [total_amount: net_amount]
          end
      end
    )
  end
end

"sales.csv"
|> Sales.read_csv()
|> Sales.parse_csv()
|> Sales.sales_tax()
|> IO.inspect()
