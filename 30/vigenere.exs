
defmodule Vigenere do
	

	def encrypt(text , key) when is_binary(text) and is_binary(key) do
			
		text
		|> String.downcase
		|> String.codepoints
		|> Vigenere.encrypt(key |> String.downcase |> String.codepoints)

	end 		

	def encrypt(text, key) when is_list(text) and is_list(key) do
		
		key_length = key 
												|> Kernel.length

		text
		|> Enum.map_reduce(0,fn

				x,acc when x == " " -> {32 , acc + 1}

				x,acc -> 

				{x
					|> :binary.first
					|> Integer.mod(97)
					|> Kernel.+(key |> Enum.at(acc |> Integer.mod(key_length)) |> :binary.first |> Integer.mod(97))
					|> Integer.mod(26)
					|> Kernel.+(97) , acc + 1}
		end)
		|> elem(0)

	end

end

"MICHI GANTE CHNOL OGICA LUNIV ERSIT Y"
|> Vigenere.encrypt("HOUGH TONHO UGHTO NHOUG HTONH OUGHT O") 
|> IO.inspect