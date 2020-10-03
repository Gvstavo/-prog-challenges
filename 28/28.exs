

defmodule Fibonacci do


	def fib_rec(n) when n == 1 or n == 0, do: n

	def fib_rec(n), do: fib_rec(n - 1) + fib_rec(n - 2) 
	
end

0..10
|> Enum.to_list
|> Enum.each(fn x -> x |> Fibonacci.fib_rec |> IO.inspect end)