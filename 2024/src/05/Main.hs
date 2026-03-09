module Main where

import Data.Set (Set)
import qualified Data.Set as Set
import Text.Read (readMaybe)

data Input = Input {rules :: [(Int, Int)], updates :: [[Int]]} deriving (Show)

splitList :: (a -> Bool) -> [a] -> [[a]]
splitList predicate list
  | (x : xs) <- list, predicate x = [] : splitList predicate xs
  | (x : xs) <- list, (y : ys) <- splitList predicate xs = (x : y) : ys
  | otherwise = [[]]

atoi :: String -> Int
atoi s = case readMaybe s :: Maybe Int of
  Just x -> x
  Nothing -> error ("not a number: " ++ s)

parseRule :: String -> (Int, Int)
parseRule line
  | [s1, s2] <- splitList (== '|') line = (atoi s1, atoi s2)
  | otherwise = error ("invalid line: " ++ line)

parseUpdate :: String -> [Int]
parseUpdate line = map atoi (splitList (== ',') line)

parseFile :: String -> Input
parseFile text =
  case splitList (== "") (lines text) of
    [ruleLines, updateLines] ->
      let ruleList = map parseRule ruleLines
          updateList = map parseUpdate updateLines
       in Input ruleList updateList
    _ -> error ("invalid input: " ++ text)

generateSet :: [(Int, Int)] -> Set (Int, Int)
generateSet [] = Set.empty
generateSet ((before, after) : rs) = Set.insert (before, after) (generateSet rs)

isForbidden :: [Int] -> Set (Int, Int) -> Bool
isForbidden [] _ = False
isForbidden (x : xs) ruleSet = any (\y -> Set.member (y, x) ruleSet) xs || isForbidden xs ruleSet

getMiddle :: [a] -> a
getMiddle list = list !! (length list `div` 2)

part1 :: [Char] -> Int
part1 text =
  let input = parseFile text
      ruleSet = generateSet (rules input)
      allowedUpdates = filter (not . (`isForbidden` ruleSet)) (updates input)
      middles = map getMiddle allowedUpdates
      total = sum middles
   in total

main :: IO ()
main = do
  text <- readFile "./input/05.txt"
  print (part1 text)
