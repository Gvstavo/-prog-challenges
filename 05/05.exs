fizzbuzz = fn 
	x when Kernel.rem(x,3) == 0 and Kernel.rem(x,5) == 0 -> IO.inspect("FizzBuzz")
	x when Kernel.rem(x,3) == 0 -> IO.inspect("Fizz")
	x when Kernel.rem(x,5)  == 0-> IO.inspect("Buzz")
	x -> IO.inspect(x)
end


1..100
|> Enum.to_list
|> Enum.each(&fizzbuzz.(&1))