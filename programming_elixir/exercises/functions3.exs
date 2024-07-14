fizz_buzz = fn
  0, 0, _ -> "FizzBuzz"
  0, _, _ -> "Fizz"
  _, 0, _ -> "Buzz"
  _, _, c -> c
end

driver = fn
  n -> fizz_buzz.(rem(n, 3), rem(n, 5), n)
end

IO.puts(driver.(10))
IO.puts(driver.(11))
IO.puts(driver.(12))
IO.puts(driver.(13))
IO.puts(driver.(14))
IO.puts(driver.(15))
IO.puts(driver.(16))
