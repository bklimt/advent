using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace advent
{
    public class Map
    {
        // The actual map.
        Tile[,] Tiles { get; set; }
        int Rows { get; set; }
        int Cols { get; set; }

        // Map of portal to its locations.
        Dictionary<string, List<Tuple<int, int>>> portalLocations = new Dictionary<string, List<Tuple<int, int>>>();

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
                                case var cc when (Char.IsUpper(cc)):
                                    map.Tiles[row, col].Type = Tile.TileType.Label;
                                    map.Tiles[row, col].Value = c;
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
                    s += Tiles[row, col].ToChar();
                }
                s += "\n";
            }
            return s.TrimEnd();
        }

        public void ComputePortals()
        {
            for (int row = 1; row < Rows - 1; row++)
            {
                for (int col = 1; col < Cols - 1; col++)
                {
                    if (Tiles[row, col].Type == Tile.TileType.Label)
                    {
                        string name = "";
                        // Figure out if this is really a portal piece or just a label.
                        if (Tiles[row + 1, col].Type == Tile.TileType.Empty)
                        {
                            name = "" + Tiles[row - 1, col].Value + Tiles[row, col].Value;
                        }
                        else if (Tiles[row - 1, col].Type == Tile.TileType.Empty)
                        {
                            name = "" + Tiles[row, col].Value + Tiles[row + 1, col].Value;
                        }
                        else if (Tiles[row, col + 1].Type == Tile.TileType.Empty)
                        {
                            name = "" + Tiles[row, col - 1].Value + Tiles[row, col].Value;
                        }
                        else if (Tiles[row, col - 1].Type == Tile.TileType.Empty)
                        {
                            name = "" + Tiles[row, col].Value + Tiles[row, col + 1].Value;
                        }
                        else
                        {
                            // This is just a label.
                        }
                        if (name.Length > 0)
                        {
                            Tiles[row, col].Type = Tile.TileType.Portal;
                            Tiles[row, col].PortalName = name;
                            if (!portalLocations.ContainsKey(name))
                            {
                                portalLocations[name] = new List<Tuple<int, int>>();
                            }
                            portalLocations[name].Add(Tuple.Create(row, col));
                        }
                    }
                }
            }
        }

        private bool MergePortals()
        {
            bool updated = false;
            foreach (var entry in portalLocations)
            {
                if (entry.Value.Count() == 1)
                {
                    continue;
                }
                if (entry.Value.Count() != 2)
                {
                    throw new Exception("portal with too many locations");
                }
                var p1 = entry.Value.First();
                var p2 = entry.Value.Last();
                var t1 = Tiles[p1.Item1, p1.Item2];
                var t2 = Tiles[p2.Item1, p2.Item2];
                if (t1.MergeLandmarks(t2))
                {
                    updated = true;
                }
                if (t2.MergeLandmarks(t1))
                {
                    updated = true;
                }
            }
            return updated;
        }

        private bool UpdateLandmarks()
        {
            bool updated = false;
            if (MergePortals())
            {
                updated = true;
            }
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
            while (UpdateLandmarks()) { }
        }

        public void PrintPortals()
        {
            foreach (var portalPosList in portalLocations)
            {
                Console.WriteLine("{0}:", portalPosList.Key);
                foreach (var portalPos in portalPosList.Value)
                {
                    Console.WriteLine("  ({0}, {1}):", portalPos.Item1, portalPos.Item2);
                    var tile = Tiles[portalPos.Item1, portalPos.Item2];
                    foreach (var portalDist in tile.landmarkDists)
                    {
                        Console.WriteLine("    {0}: {1}", portalDist.Key, portalDist.Value);
                    }
                }
            }
        }

        public void Search()
        {
        }
    }
}
