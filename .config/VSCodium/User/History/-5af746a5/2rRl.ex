defmodule FirstMix.Recursion.PrintDigits do
  def up_to(num) do
    up_to_recursive(num)
  end

  defp up_to_recursive(num, counter \\ 0)
  defp up_to_recursive(num, _counter) when num <= 0 do
    0
  end
  defp up_to_recursive(counter, counter) do
    IO.puts(counter)
  end

  defp up_to_recursive(num, counter) do
    IO.puts(counter)
    new_counter = counter + 1
    up_to_recursive(num, new_counter)
  end


  def down_from(num) when num <= 0 do
    IO.puts(0)
  end

  def down_from(num) do
    IO.puts(num)
    new_num = num - 1
    down_from(new_num)
  end

end


defmodule Recursion.PrintDigitsb do

end
