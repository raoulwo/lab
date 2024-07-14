prefix = fn
  pre -> fn str -> "#{pre} #{str}" end
end

mrs = prefix.("Mrs.")
mr = prefix.("Mr.")

IO.puts(mrs.("Smith"))
IO.puts(mr.("White"))
