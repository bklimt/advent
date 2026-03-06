module Main where

data Grid = Grid {values :: [[Char]], rows :: Int, columns :: Int} deriving (Show)

parseLine :: String -> [Char]
parseLine line = line

parseFile :: String -> Grid
parseFile text =
  Grid vs (length vs) (length (head vs))
  where
    vs = map parseLine (lines text)

valueAt :: Grid -> Int -> Int -> Maybe Char
valueAt grid row column
  | row < 0 = Nothing
  | column < 0 = Nothing
  | row >= rows grid = Nothing
  | column >= columns grid = Nothing
  | otherwise = Just ((values grid !! row) !! column)

checkAtWithDelta :: Grid -> Int -> Int -> Int -> Int -> [Char] -> Int
checkAtWithDelta _ _ _ _ _ "" = 1
checkAtWithDelta grid row column rowDelta columnDelta str
  | Nothing <- current = 0
  | Just letter <- current, letter /= head str = 0
  | otherwise = checkAtWithDelta grid (row + rowDelta) (column + columnDelta) rowDelta columnDelta (tail str)
  where
    current = valueAt grid row column

checkAt :: Grid -> Int -> Int -> [Char] -> Int
checkAt grid row column str =
  let a = checkAtWithDelta grid row column 1 0 str
      b = checkAtWithDelta grid row column (-1) 0 str
      c = checkAtWithDelta grid row column 0 1 str
      d = checkAtWithDelta grid row column 0 (-1) str
      e = checkAtWithDelta grid row column 1 1 str
      f = checkAtWithDelta grid row column 1 (-1) str
      g = checkAtWithDelta grid row column (-1) (-1) str
      h = checkAtWithDelta grid row column (-1) 1 str
   in a + b + c + d + e + f + g + h

search :: Grid -> Int -> Int -> [Char] -> Int
search grid row column str
  | row >= rows grid = 0
  | column >= columns grid = search grid (row + 1) 0 str
  | otherwise = checkAt grid row column str + search grid row (column + 1) str

getXAt :: Grid -> Int -> Int -> (Maybe Char, Maybe Char, Maybe Char, Maybe Char)
getXAt grid row column =
  let f drow dcol = valueAt grid (row + drow) (column + dcol)
   in (f (-1) (-1), f (-1) 1, f 1 (-1), f 1 1)

checkAt2 :: Grid -> Int -> Int -> Bool
checkAt2 grid row column
  | Just letter <- current, letter /= 'A' = False
  | (Just 'M', Just 'M', Just 'S', Just 'S') <- x = True
  | (Just 'M', Just 'S', Just 'M', Just 'S') <- x = True
  | (Just 'S', Just 'S', Just 'M', Just 'M') <- x = True
  | (Just 'S', Just 'M', Just 'S', Just 'M') <- x = True
  | otherwise = False
  where
    current = valueAt grid row column
    x = getXAt grid row column

countAt2 :: Grid -> Int -> Int -> Int
countAt2 grid row column = if checkAt2 grid row column then 1 else 0

search2 :: Grid -> Int -> Int -> Int
search2 grid row column
  | row >= (rows grid - 1) = 0
  | column >= (columns grid - 1) = search2 grid (row + 1) 1
  | otherwise = countAt2 grid row column + search2 grid row (column + 1)

part1 :: [Char] -> Int
part1 text =
  let grid = parseFile text
   in search grid 0 0 "XMAS"

part2 :: [Char] -> Int
part2 text =
  let grid = parseFile text
   in search2 grid 1 1

main :: IO ()
main = do
  text <- readFile "./input/04.txt"
  print (part1 text)
  print (part2 text)
