defmodule Exgui do
  @moduledoc """
  Documentation for `Exgui`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Exgui.hello()
      :world

  """
  def hello do
    :world
  end

  use Rustler, otp_app: :exgui, crate: "exgui"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
