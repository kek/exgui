defmodule ExguiTest do
  use ExUnit.Case
  doctest Exgui

  test "greets the world" do
    assert Exgui.hello() == :world
  end
end
