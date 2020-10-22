
# Sieve of Eratosthenes
require Integer

defmodule Prime do

	def sieve(n , _acc \\ 0) when is_integer(n), do: 2..n |>Enum.to_list |> sieve(2)

	def sieve(n, acc) when acc > length(n), do: n

	def sieve(n, acc) do

		n
		|> Enum.flat_map(fn

			x when x == acc -> [x]

			x when rem(x,acc) == 0 -> []

			x -> [x]

		end)
		|> sieve(acc + 1)

	end
	
end

Prime.sieve(10) |> IO.inspect