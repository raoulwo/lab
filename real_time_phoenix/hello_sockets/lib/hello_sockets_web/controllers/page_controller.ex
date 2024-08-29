defmodule HelloSocketsWeb.PageController do
  use HelloSocketsWeb, :controller

  @fake_user_id 1

  def home(conn, _params) do
    conn
    |> assign(:auth_token, generate_auth_token(conn, @fake_user_id))
    |> assign(:user_id, @fake_user_id)
    |> render(:home, layout: false)
  end

  defp generate_auth_token(conn, user_id) do
    Phoenix.Token.sign(conn, "salt identifier", user_id)
  end
end
