defmodule ExguiTest do
  use ExUnit.Case
  doctest Exgui

  test "greets the world" do
    assert Exgui.hello() == :world
  end

  test "adding by Rustler NIF" do
    assert Exgui.add(1, 2) == 3
  end
end
