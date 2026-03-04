module Main where

import Text.Read (readMaybe)
import Text.Regex.PCRE

atoi :: String -> Int
atoi s = case readMaybe s :: Maybe Int of
  Just x -> x
  Nothing -> error ("not a number: " ++ s)

processMul :: [String] -> Int
processMul [_, x, y] = atoi x * atoi y
processMul [text] = error ("invalid entry: " ++ text)
processMul _ = error "empty mul?"

part1 :: String -> Int
part1 text =
  let pattern = "mul\\(([0-9]{1,3}),([0-9]{1,3})\\)" :: String
      allMatches = text =~ pattern :: [[String]]
      numbers = map processMul allMatches
   in sum numbers

main :: IO ()
main = do
  text <- readFile "./input/03.txt"
  print (part1 text)
