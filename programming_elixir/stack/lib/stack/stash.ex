defmodule Stack.Stash do
  use GenServer

  def start_link(initial_stack) do
    GenServer.start_link(__MODULE__, initial_stack, name: __MODULE__)
  end

  def get do
    GenServer.call(__MODULE__, :get)
  end

  def update(new_stack) do
    GenServer.cast(__MODULE__, {:update, new_stack})
  end

  def init(initial_stack) do
    {:ok, initial_stack}
  end

  def handle_call(:get, _from, current_stack) do
    {:reply, current_stack, current_stack}
  end

  def handle_cast({:update, new_stack}, _current_stack) do
    {:noreply, new_stack}
  end
end
