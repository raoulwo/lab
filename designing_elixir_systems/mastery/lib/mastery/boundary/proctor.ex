defmodule Mastery.Boundary.Proctor do
  use GenServer
  require Logger
  alias Mastery.Boundary.{QuizManager, QuizSession}

  def schedule_quiz(proctor \\ __MODULE__, quiz, templates, start_at, end_at, notify_pid) do
    quiz = %{
      fields: quiz,
      templates: templates,
      start_at: start_at,
      end_at: end_at,
      notify_pid: notify_pid
    }

    GenServer.call(proctor, {:schedule_quiz, quiz})
  end

  def start_link(options \\ []) do
    GenServer.start_link(__MODULE__, [], options)
  end

  @impl GenServer
  def init(quizzes) do
    {:ok, quizzes}
  end

  @impl GenServer
  def handle_call({:schedule_quiz, quiz}, _from, quizzes) do
    now = DateTime.utc_now()

    ordered_quizzes =
      [quiz | quizzes]
      |> start_quizzes(now)
      |> Enum.sort(fn a, b -> date_time_less_than_or_equal?(a.start_at, b.start_at) end)

    build_reply_with_timeout({:reply, :ok}, ordered_quizzes, now)
  end

  defp start_quizzes(quizzes, now) do
    {ready, not_ready} =
      Enum.split_while(quizzes, fn quiz ->
        date_time_less_than_or_equal?(quiz.start_at, now)
      end)

    Enum.each(ready, fn quiz -> start_quiz(quiz, now) end)
    not_ready
  end

  defp date_time_less_than_or_equal?(a, b) do
    DateTime.compare(a, b) in ~w[lt eq]a
  end

  def start_quiz(quiz, now) do
    Logger.info("Starting quiz #{quiz.fields.title}...")
    notify_start(quiz)
    QuizManager.build_quiz(quiz.fields)
    Enum.each(quiz.templates, &Mastery.Boundary.QuizManager.add_template(quiz.fields.title, &1))
    timeout = DateTime.diff(quiz.end_at, now, :millisecond)
    Process.send_after(self(), {:end_quiz, quiz.fields.title, quiz.notify_pid}, timeout)
  end

  defp notify_start(%{notify_pid: nil}), do: nil

  defp notify_start(quiz) do
    send(quiz.notify_pid, {:started, quiz.fields.title})
  end

  defp build_reply_with_timeout(reply, quizzes, now) do
    reply
    |> append_state(quizzes)
    |> maybe_append_timeout(quizzes, now)
  end

  defp append_state(tuple, quizzes), do: Tuple.append(tuple, quizzes)

  defp maybe_append_timeout(tuple, [], _now), do: tuple

  defp maybe_append_timeout(tuple, quizzes, now) do
    timeout =
      quizzes
      |> hd()
      |> Map.fetch!(:start_at)
      |> DateTime.diff(now, :millisecond)

    Tuple.append(tuple, timeout)
  end

  @impl GenServer
  def handle_info(:timeout, quizzes) do
    now = DateTime.utc_now()
    remaining_quizzes = start_quizzes(quizzes, now)
    build_reply_with_timeout({:noreply}, remaining_quizzes, now)
  end

  @impl GenServer
  def handle_info({:end_quiz, title, notify_pid}, quizzes) do
    QuizManager.remove_quiz(title)

    title
    |> QuizSession.active_sessions_for()
    |> QuizSession.end_sessions()

    Logger.info("Stopped quiz #{title}.")
    notify_stopped(notify_pid, title)
    handle_info(:timeout, quizzes)
  end

  defp notify_stopped(nil, _title), do: nil
  defp notify_stopped(pid, title), do: send(pid, {:stopped, title})
end
