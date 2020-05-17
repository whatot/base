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

  test "push sequence" do
    Portal.Door.start_link(:pd3)
    Portal.Door.push(:pd3, 1)
    Portal.Door.push(:pd3, 2)
    assert Portal.Door.get(:pd3) == [2, 1]
    Portal.Door.push(:pd3, 3)
    assert Portal.Door.get(:pd3) == [3, 2, 1]
  end
end
