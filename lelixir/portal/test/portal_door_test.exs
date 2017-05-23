defmodule PortalDoorTest do
  use ExUnit.Case
  doctest Portal.Door

  test "init door" do
    Portal.Door.start_link(:pd0)
    assert Portal.Door.get(:pd0) == []
  end

  test "push and get" do
    Portal.Door.start_link(:pd1)
    Portal.Door.push(:pd1, "what")
    assert Portal.Door.get(:pd1) == ["what"]
  end

  test "push and pop" do
    Portal.Door.start_link(:pd2)
    Portal.Door.push(:pd2, "hello")
    Portal.Door.push(:pd2, "world")
    assert Portal.Door.pop(:pd2) == {:ok, "world"}
    assert Portal.Door.pop(:pd2) == {:ok, "hello"}
    assert Portal.Door.pop(:pd2) == :error
  end
end
