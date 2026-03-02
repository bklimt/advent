module Main where

import Data.List (sort)
import Text.Read (readMaybe)

atoi :: String -> Int
atoi s = case readMaybe s :: Maybe Int of
  Just x -> x
  Nothing -> error ("not a number: " ++ s)

parseLine :: String -> (Int, Int)
parseLine line =
  let numbers = map atoi (words line)
   in case numbers of
        [first, second] -> (first, second)
        _ -> error ("invalid line: " ++ line)

parseFile :: String -> [(Int, Int)]
parseFile text = map parseLine (lines text)

part1 :: String -> Int
part1 text =
  let numbers = parseFile text
      xs = sort (map fst numbers)
      ys = sort (map snd numbers)
      diffs = map abs (zipWith (-) xs ys)
   in sum diffs

-- Returns an assoc list that maps number to its frequency. The input list must be sorted.
countElements :: (Num b, Eq a) => [a] -> [(a, b)]
countElements [] = []
countElements [x] = [(x, 1)]
countElements (x : xs)
  | x == fst first = (x, snd first + 1) : tail counts
  | otherwise = (x, 1) : counts
  where
    counts = countElements xs
    first = head counts

-- Looks up an item in an assoc list. Returns 0 if it's not found.
findCount :: (Num a, Eq t) => t -> [(t, a)] -> a
findCount _ [] = 0
findCount x ((y, count) : ys)
  | x == y = count
  | otherwise = findCount x ys

part2 :: String -> Int
part2 text =
  let numbers = parseFile text
      xs = map fst numbers
      ys = sort (map snd numbers)
      counts = countElements ys
      scores = [x * findCount x counts | x <- xs]
   in sum scores

main :: IO ()
main = do
  text <- readFile "./input/01.txt"
  print (part1 text)
  print (part2 text)
