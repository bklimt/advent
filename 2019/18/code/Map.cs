using System;
using System.Collections.Generic;
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

        public void Search()
        {
            // First, build an adjacency matrix with the set of shortest paths from
            // each key to each key, along with what keys are needed to make that path.
            var indexToLandmark = new List<char>(landmarks.Keys);
            var landmarkToIndex = new Dictionary<char, int>();
            var count = indexToLandmark.Count;
            for (int i = 0; i < count; i++)
            {
                landmarkToIndex[indexToLandmark[i]] = i;
            }
            var matrix = new PathSet[count, count];
            for (int i = 0; i < count; i++)
            {
                for (int j = 0; j < count; j++)
                {
                    matrix[i, j] = new PathSet();
                }
            }
            // Start with everything directly reachable.
            for (int i = 0; i < count; i++)
            {
                var reachable = GetReachableLandmarks(indexToLandmark[i]);
                foreach (var t in reachable)
                {
                    var j = landmarkToIndex[t.Key];
                    var path = new Path();
                    path.Distance = t.Value;
                    matrix[i, j].Paths.Add(path);
                }
            }
            // Now build up all the derivative steps.
            for (int m = 0; m < count; m++)
            {
                Console.WriteLine("Passes: {0} of {1}", m, count);
                for (int i = 0; i < count; i++)
                {
                    Console.WriteLine("Row: {0} of {1} = {2}", i, count, indexToLandmark[i]);
                    for (int j = 0; j < count; j++)
                    {
                        Console.WriteLine("Column: {0} of {1} = {2}", j, count, indexToLandmark[j]);
                        if (i == j)
                        {
                            continue;
                        }
                        var op = matrix[i, j];
                        for (int k = 0; k < count; k++)
                        {
                            // Console.WriteLine("Intermediate: {0} of {1}", k, count);
                            if (i == k || j == k)
                            {
                                continue;
                            }
                            // TODO: If k is a lock, add it to keysNeeded.
                            // Is there a shorter path from i to j through k?
                            var ps1 = matrix[i, k];
                            var ps2 = matrix[k, j];
                            if (ps1.Paths.Count > 0 && ps2.Paths.Count > 0)
                            {
                                foreach (var p1 in ps1.Paths)
                                {
                                    foreach (var p2 in ps2.Paths)
                                    {
                                        op.Paths.Add(MergePaths(p1, p2));
                                        op.ReducePaths();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}