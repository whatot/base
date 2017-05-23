defmodule PortalTest do
  use ExUnit.Case
  doctest Portal

  test "create door with shoot" do
    Portal.shoot(:c0)
    assert Portal.Door.get(:c0) == []
  end

  test "create portal with transfer" do
    Portal.shoot(:c1)
    Portal.shoot(:c2)
    p1 = Portal.transfer(:c1, :c2, [1,2,3,4,5])
    assert p1.left == :c1
    assert p1.right == :c2
    assert Portal.Door.get(p1.left) == [5, 4, 3, 2, 1]
  end

  test "push right and left" do
    Portal.shoot(:c3)
    Portal.shoot(:c4)
    p2 = Portal.transfer(:c3, :c4, [1,2,3,4,5])
    assert Portal.Door.get(p2.left) == [5, 4, 3, 2, 1]

    Portal.push_right(p2)
    assert Portal.Door.get(p2.left) == [4, 3, 2, 1]
    assert Portal.Door.get(p2.right) == [5]

    Portal.push_right(p2)
    assert Portal.Door.get(p2.left) == [3, 2, 1]
    assert Portal.Door.get(p2.right) == [4, 5]

    Portal.push_left(p2)
    assert Portal.Door.get(p2.left) == [4, 3, 2, 1]
    assert Portal.Door.get(p2.right) == [5]
  end

end
