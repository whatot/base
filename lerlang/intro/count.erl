-module(count).
-export([countdown/1, countup/1, factorial_1/1, factorial_2/1]).

countdown(From) when From > 0 ->
    io:format("~w!~n", [From]),
    countdown(From-1);
countdown(_) ->
    io:format("blastoff!~n").

countup(Limit) ->
    countup(1, Limit).
countup(Count, Limit) when Count =< Limit ->
    io:format("~w!~n", [Count]),
    countup(Count+1, Limit);
countup(_, _) ->
    io:format("Finished.~n").

factorial_1(N) when N > 1 ->
    N * factorial_1(N-1);
factorial_1(N) when N =< 1 ->
    1.

factorial_2(N) ->
    factorial_2(1, N, 1).
factorial_2(Current, N, Result) when Current =< N ->
    factorial_2(Current+1, N, Result*Current);
factorial_2(_, _, Result) ->
    Result.
