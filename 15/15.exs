# collatz conjecture
#
#    If the number is even, divide it by two.
#    If the number is odd, triple it and add one.#
#
require Integer

defmodule Math do

	def collatz(n) when n <= 1, do: []

	def collatz(n), do: collatz(n , [])

	def collatz(n , computations) when n == 1 , do: computations 

	def collatz(n , computations ) when Integer.is_odd(n), do: collatz(3*n+1 , computations ++ [3*n+1])

	def collatz(n,computations) when Integer.is_even(n), do: 	collatz(Integer.floor_div(n,2), computations ++ [Integer.floor_div(n ,2)])		#collatz(n / 2 , computations ++ [n / 2])
	
end



Math.collatz(19)
|> IO.inspect

