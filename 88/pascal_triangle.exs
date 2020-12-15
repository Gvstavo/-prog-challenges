defmodule Math do


	def pascal_triangle(n, k)  when n == k or k == 0 or n == 0, do:  1

	def pascal_triangle(n , k), do:  pascal_triangle(n - 1 , k - 1) + pascal_triangle(n - 1 , k)

	def pascal_triangle(n , t  , k ) when t == n and k == n, do: pascal_triangle(t , k) |> IO.write

	def pascal_triangle(n , t , k) when t == k do

		pascal_triangle(t , k) |> IO.puts
		pascal_triangle(n , t + 1 , 0)

	end   

	def pascal_triangle(n , t , k) when  t > k do
		
		pascal_triangle(t , k) |> IO.write
		" " |> IO.write
		pascal_triangle(n , t , k + 1)

	end

	def pascal_triangle(n), do: pascal_triangle(n , 0 , 0)

end

Math.pascal_triangle(3) 