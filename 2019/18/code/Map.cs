using System;
using System.Collections.Generic;
using System.IO;

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

        public IEnumerable<char> GetReachableLandmarks(char start)
        {
            var t = landmarks[start];
            var row = t.Item1;
            var col = t.Item2;
            var keys = Tiles[row, col].landmarkDists.Keys;
            return keys;
        }

        public void Search()
        {
            char current = '@';
            var reachable = GetReachableLandmarks(current);
            foreach (var key in reachable)
            {
                Console.WriteLine("Reachable: {0}", key);
            }
        }
    }
}