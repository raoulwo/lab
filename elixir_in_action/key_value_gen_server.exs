defmodule KeyValueStore do
  use GenServer

  def start do
    # GenServer.start/2 only returns after init/1 has finished.
    # The client starting the genserver blocks until the initialization
    # has completed. The option `:name` registers a local name for the
    # GenServer so that we don't need to address it via PID, by
    # convention we use the module name since it is an atom.
    GenServer.start(__MODULE__, %{}, name: __MODULE__)
  end

  def put(key, value) do
    GenServer.cast(__MODULE__, {:put, key, value})
  end

  def get(key) do
    # GenServer.call/2 times out after 5 seconds by default.
    GenServer.call(__MODULE__, {:get, key})
  end

  # The `@impl` attribute leads to compile-time warnings should a function
  # you define not satisfy a behaviour contract.
  @impl GenServer
  def init(initial_state) do
    :timer.send_interval(5000, :cleanup)
    {:ok, initial_state}

    # Besides `{:ok, initial_state}` init/1 can also return the following:
    #
    #  * `{:stop, reason}` should be returned when you can't proceed because
    #    of an error. GenServer.start/3 will then return {:error, reason}
    #
    #  * `:ignore` should be returned when stopping the server is the normal
    #    thing to do in given situation. GenServer.start/3 will return `:ignore`.
  end

  # The handle_* functions can return `{:stop, reason, new_state}` which
  # causes the GenServer to stop, handle_call/2 can additionally return a response
  # in form of `{:stop, reason, response, new_state}`. If the termination is part
  # of the standard workflow, return the atom `:normal` instead.

  # Before a GenServer is terminated it calls the terminate/2 function, here you
  # can perform cleanup if needed.

  # The function GenServer.stop/3 can be used to manually stop the server

  @impl GenServer
  def handle_info(:cleanup, state) do
    # The handle_info/2 callback is used for messages that
    # aren't GenServer specific.
    IO.puts("performing cleanup...")
    {:noreply, state}
  end

  @impl GenServer
  def handle_cast({:put, key, value}, state) do
    {:noreply, Map.put(state, key, value)}
  end

  @impl GenServer
  def handle_call({:get, key}, _, state) do
    {:reply, Map.get(state, key), state}
  end
end
