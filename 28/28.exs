

defmodule Fibonacci do


	def fib_rec(n) when n == 1 or n == 0, do: n

	def fib_rec(n), do: fib_rec(n - 1) + fib_rec(n - 2) 

	def phi, do: ( 1 + :math.sqrt(5)) / 2

	def fib_by_golden_ratio(n), do: (:math.pow(phi , n) / :math.sqrt(5) + 1/2) |> Kernel.trunc

	
end

0..10
|> Enum.to_list
|> Enum.each(fn x -> x |> Fibonacci.fib_by_golden_ratio |> IO.inspect end)