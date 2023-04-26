defmodule ExguiTest do
  use ExUnit.Case
  doctest Exgui

  test "adding by Rustler NIF" do
    assert Exgui.add(1, 2) == 3
  end

  test "greets the world" do
    result = Exgui.spawn_external_process(self())
    assert result == {}

    assert_receive "Hello world"
  end

  test "use a resource" do
    resource = Exgui.make_resource()
    assert Exgui.read_resource(resource) == 42
  end

  test "use a channel" do
    channel = Exgui.make_channel(self())
    Exgui.send_on_channel(channel, 101)
    assert_receive 101
  end
end
