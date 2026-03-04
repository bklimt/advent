module Main where

import Text.Read (readMaybe)

atoi :: String -> Int
atoi s = case readMaybe s :: Maybe Int of
  Just x -> x
  Nothing -> error ("not a number: " ++ s)

parseLine :: String -> [Int]
parseLine line = map atoi (words line)

parseFile :: String -> [[Int]]
parseFile text = map parseLine (lines text)

unsafeReason :: [Int] -> Maybe String
unsafeReason [] = Nothing
unsafeReason (x : xs)
  | (y : z : _) <- xs, signum (z - y) /= signum (y - x) = Just "not monotonic"
  | (y : _) <- xs, signum (y - x) == 0 = Just "delta of zero"
  | (y : _) <- xs, abs (y - x) > 3 = Just "delta > 3"
  | otherwise = unsafeReason xs

isSafe :: [Int] -> Bool
isSafe record
  | Just _ <- reason = False
  | Nothing <- reason = True
  where
    reason = unsafeReason record

part1 :: String -> Int
part1 text =
  let records = parseFile text
      safeRecords = filter isSafe records
      safeCount = length safeRecords
   in safeCount

main :: IO ()
main = do
  text <- readFile "./input/02.txt"
  print (part1 text)
