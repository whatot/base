defmodule Portal do
  @moduledoc """
  Documentation for Portal.
  """

  defstruct [:left, :right]

  @doc """
  Start transfering `data` from `left` to `right`.
  """
  def transfer(left, right, data) do
    # First add all data to the portal on the left
    for item <- data do
      Portal.Door.push(left, item)
    end

    # Returns a portal struct we will use next
    %Portal{left: left, right: right}
  end

  @doc """
  Push data to the right in the given `portal`.
  """
  def push_right(portal) do
    push_with_direction(portal, :right)
    portal
  end

  @doc """
  Push data to the left in the given `portal`.
  """
  def push_left(portal) do
    push_with_direction(portal, :left)
    portal
  end

  defp push_with_direction(portal, direction) do
    {src, dest} =
      case direction do
        :left -> {portal.right, portal.left}
        :right -> {portal.left, portal.right}
        _ -> raise "invalid direction: " <> direction
      end

    case Portal.Door.pop(src) do
      :error -> :ok
      {:ok, h} -> Portal.Door.push(dest, h)
    end

    # Let's return the portal itself
    portal
  end

  @doc """
  Shoot a new door with given `color`.
  """
  def shoot(color) do
    Supervisor.start_child(Portal.Supervisor, [color])
  end
end

defimpl Inspect, for: Portal do
  def inspect(%Portal{left: left, right: right}, _) do
    left_door = inspect(left)
    right_door = inspect(right)

    left_data = inspect(Enum.reverse(Portal.Door.get(left)))
    right_data = inspect(Portal.Door.get(right))

    max = max(String.length(left_door), String.length(left_data))

    """
    #Portal<
      #{String.rjust(left_door, max)} <=> #{right_door}
      #{String.rjust(left_data, max)} <=> #{right_data}
    >
    """
  end
end
