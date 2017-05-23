# Portal

simple example project to learn elixir,
[origin source](!http://howistart.org/posts/elixir/1/index.html)

add `credo` and `dogma` to flychek and lint code.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `portal` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [{:portal, "~> 0.1.0"}]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/portal](https://hexdocs.pm/portal).

## Usage

### Portal transfers in one

run ``iex -S mix`` under the root of project:

```elixir
Portal.shoot(:orange)
Portal.shoot(:blue)
portal = Portal.transfer(:orange, :blue, [1,2,3,4])
Portal.push_right(portal)
Portal.push_left(portal)
```

### Distributed transfers in two

1. run `iex --sname room1 --cookie secret -S mix` for room1 in project

```
Portal.shoot(:orange)
```

2. run `iex --sname room2 --cookie secret -S mix` for room2 in project

```
Portal.shoot(:blue)
orange = {:orange, :"room1@pc"}
blue = {:blue, :"room2@pc"}
portal = Portal.transfer(orange, blue, [1,2,3,4,5])
Portal.push_right(portal)
Portal.push_right(portal)
Portal.push_left(portal)
```
