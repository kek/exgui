defmodule ExguiTest do
  use ExUnit.Case
  doctest Exgui

  test "greets the world" do
    result = Exgui.hello(self())
    IO.puts("Hello returned #{inspect(result)}")
    assert result == {}

    receive do
      x -> IO.puts("Received #{inspect(x)}")
    end
  end

  test "adding by Rustler NIF" do
    assert Exgui.add(1, 2) == 3
  end
end
