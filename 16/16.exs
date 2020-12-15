defmodule Str do

	
	def reverse(str) when is_binary(str) , do: str |> String.codepoints |> Str.reverse

	def reverse(str) when is_list(str), do: str |> Enum.reduce([], fn x,acc -> [x | acc] end) |> Enum.join	

end


"String de teste" 
|> Str.reverse
|> IO.inspect