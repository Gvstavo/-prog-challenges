


defmodule Math do
	

	def factorial(n) when n <= 0, do: 1

	def factorial(n) when n == 1, do: n

	def factorial(n), do: n * factorial(n - 1)   





end


Math.factorial(9) 
|> IO.inspect