using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.IO;
using System.Linq;

namespace code
{
    public class Map
    {
        // The player's position.
        int Row { get; set; }
        int Col { get; set; }

        // The actual map.
        Tile[,] Tiles { get; set; }
        int Rows { get; set; }
        int Cols { get; set; }

        // Landmarks that are navigable.
        Dictionary<char, Tuple<int, int>> landmarks = new Dictionary<char, Tuple<int, int>>();
        int KeyCount { get; set; }

        public static Map ParseMap(string path)
        {
            var lines = File.ReadAllLines(path);
            int rows = lines.Length;
            int cols = lines[0].Length;
            var map = new Map(rows, cols);
            for (var row = 0; row < rows; row++)
            {
                {
                    if (lines[row].Length != cols)
                    {
                        throw new Exception("invalid line length");
                    }
                    for (var col = 0; col < cols; col++)
                    {
                        {
                            var c = lines[row][col];
                            switch (c)
                            {
                                case '#':
                                    map.Tiles[row, col].Type = Tile.TileType.Wall;
                                    break;
                                case '.':
                                    map.Tiles[row, col].Type = Tile.TileType.Empty;
                                    break;
                                case '@':
                                    map.landmarks['@'] = Tuple.Create(row, col);
                                    map.Row = row;
                                    map.Col = col;
                                    map.Tiles[row, col].Type = Tile.TileType.Empty;
                                    break;
                                case var cc when (Char.IsLower(cc)):
                                    map.landmarks[c] = Tuple.Create(row, col);
                                    map.Tiles[row, col].Type = Tile.TileType.Key;
                                    map.Tiles[row, col].Value = c;
                                    break;
                                case var cc when (Char.IsUpper(cc)):
                                    map.landmarks[c] = Tuple.Create(row, col);
                                    map.Tiles[row, col].Type = Tile.TileType.Lock;
                                    map.Tiles[row, col].Value = Char.ToLower(c);
                                    break;
                            }
                        }
                    }
                }
            }
            return map;
        }

        private Map(int rows, int cols)
        {
            Rows = rows;
            Cols = cols;
            Tiles = new Tile[rows, cols];
            for (int i = 0; i < rows; i++)
            {
                for (int j = 0; j < cols; j++)
                {
                    Tiles[i, j] = new Tile();
                }
            }
        }

        override public string ToString()
        {
            var s = "";
            for (var row = 0; row < Rows; row++)
            {
                for (var col = 0; col < Cols; col++)
                {
                    if (row == Row && col == Col)
                    {
                        s += '@';
                    }
                    else
                    {
                        s += Tiles[row, col].ToChar();
                    }
                }
                s += "\n";
            }
            return s.TrimEnd();
        }

        private bool UpdateLandmarks()
        {
            bool updated = false;
            for (var row = 0; row < Rows; row++)
            {
                for (var col = 0; col < Cols; col++)
                {
                    if (row > 0)
                    {
                        if (Tiles[row, col].UpdateLandmarks(Tiles[row - 1, col]))
                        {
                            updated = true;
                        }
                    }
                    if (col > 0)
                    {
                        if (Tiles[row, col].UpdateLandmarks(Tiles[row, col - 1]))
                        {
                            updated = true;
                        }
                    }
                    if (row < Rows - 1)
                    {
                        if (Tiles[row, col].UpdateLandmarks(Tiles[row + 1, col]))
                        {
                            updated = true;
                        }
                    }
                    if (col < Cols - 1)
                    {
                        if (Tiles[row, col].UpdateLandmarks(Tiles[row, col + 1]))
                        {
                            updated = true;
                        }
                    }
                }
            }
            return updated;
        }

        public void ComputeLandmarks()
        {
            Tiles[Row, Col].landmarkDists['@'] = 0;
            while (UpdateLandmarks()) { }
            KeyCount = (from entry in landmarks
                        where Char.IsLower(entry.Key)
                        select entry).Count();
        }

        public Dictionary<char, int> GetReachableLandmarks(char start)
        {
            var t = landmarks[start];
            var row = t.Item1;
            var col = t.Item2;
            return Tiles[row, col].landmarkDists;
        }

        class Path
        {
            public int Distance { get; set; }
            public HashSet<char> KeysNeeded { get; set; } = new HashSet<char>();
        }

        private static Path MergePaths(Path p1, Path p2)
        {
            Path p = new Path();
            p.Distance = p1.Distance + p2.Distance;
            p.KeysNeeded.UnionWith(p1.KeysNeeded);
            p.KeysNeeded.UnionWith(p2.KeysNeeded);
            return p;
        }

        class PathSet
        {
            public List<Path> Paths { get; set; } = new List<Path>();

            public void ReducePaths()
            {
                // For every path, if there's another path that's shorter and
                // requires fewer keys, remove it.
                for (int i = Paths.Count - 1; i >= 0; i--)
                {
                    bool remove = false;
                    for (int j = 0; j < Paths.Count; j++)
                    {
                        if (i == j)
                        {
                            continue;
                        }
                        if (Paths[j].Distance < Paths[i].Distance)
                        {
                            // J is shorter than I, but does it need fewer or equal locks?
                            if (Paths[j].KeysNeeded.IsSubsetOf(Paths[i].KeysNeeded))
                            {
                                remove = true;
                                break;
                            }
                        }
                    }
                    if (remove)
                    {
                        Paths.RemoveAt(i);
                    }
                }

            }
        }

        public int Search(string path, int dist, int best, ImmutableHashSet<char> keys, int maxPath, int maxDupes)
        {
            // Don't recurse too deeply.
            if (path.Length > maxPath)
            {
                return -1;
            }

            if (keys.Count == KeyCount)
            {
                return dist;
            }

            Console.WriteLine("keys={0}, dist={1}, path={2}, dupes={3}, best={4}", keys.Count, dist, path, maxDupes, best);

            char current = path.Last();

            // Chack the current spot for whether we should update state.
            bool gotKey = false;
            if (Char.IsLower(current))
            {
                // We picked up a key maybe.
                if (!keys.Contains(current))
                {
                    gotKey = true;
                    keys = keys.Add(current);
                }
            }

            // Get the set of every landmark reachable from this spot.
            var reachable = GetReachableLandmarks(current);

            // Remove any lock that we don't have a key for.
            // Also remove current.
            var options = from entry in reachable
                          orderby entry.Value
                          where (!Char.IsUpper(entry.Key) || keys.Contains(Char.ToLower(entry.Key))) &&
                                entry.Key != current &&
                                path.Count(c => c == entry.Key) <= maxDupes
                          select entry;

            for (int dupes = 0; dupes <= maxDupes; dupes++)
            {
                foreach (var entry in options)
                {
                    var c = entry.Key;
                    var d = entry.Value;
                    // Don't just move back and forth.
                    if (path.Length > 2 && c == path[path.Length - 3])
                    {
                        continue;
                    }
                    // Don't go right back unless you came here to pick up a key.
                    if (!gotKey && path.Length > 1 && c == path[path.Length - 2])
                    {
                        continue;
                    }
                    // Cut off the path if it's longer than the best path so far.
                    if (best != -1 && dist + d > best)
                    {
                        continue;
                    }
                    // Okay, this is a valid path. Traverse it.
                    int result = Search(path + c, dist + d, best, keys, maxPath, dupes);
                    // It didn't work out, probably because of pruning.
                    if (result == -1)
                    {
                        continue;
                    }
                    if (best == -1 || result < best)
                    {
                        Console.WriteLine("Best: {0} = {1}", path + c, result);
                        best = result;
                    }
                }
            }

            return best;
        }

        public void Search()
        {
            int best = -1;
            int maxPath = 100;
            for (int i = 0; i < 10; i++)
            {
                int result = Search("@", 0, best, ImmutableHashSet<char>.Empty, maxPath, i);
                if (result != -1 && (best == -1 || result < best))
                {
                    best = result;
                }
            }
        }
    }
}