using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.IO;
using System.Linq;

namespace code
{
    using SearchStateDictionary =
        System.Collections.Generic.SortedDictionary<Tuple<double, string>, Map.SearchState>;

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

        public struct SearchState
        {
            // The hierarchical search state.
            public string Path;
            public int Distance;
            public ImmutableHashSet<char> Keys;

            public double Score
            {
                get
                {
                    return Math.Sqrt(Distance) / (double)(Keys.Count() + 1);
                }
            }

            override public string ToString()
            {
                return String.Format("keys={0}, dist={1}, path={2}",
                                     Keys.Count, Distance, Path);
            }
        }

        // A priority queue for the states to search.
        private class SearchStateQueue
        {
            private int maxKeyCount = 0;
            private readonly int MAX_ENTRIES = 2000000;

            private SearchStateDictionary states = new SearchStateDictionary();

            public void Add(SearchState state)
            {
                if (state.Keys.Count() > maxKeyCount)
                {
                    maxKeyCount = state.Keys.Count();
                }

                states.Add(Tuple.Create(state.Score, state.Path), state);

                // Make sure the queue doesn't get too huge.
                if (states.Count > MAX_ENTRIES)
                {
                    TrimQueue();
                }
            }

            public SearchState Pop()
            {
                var entry = states.ElementAt(0);
                states.Remove(entry.Key);
                return entry.Value;
            }

            private void TrimQueue()
            {
                // The queue is just too large. Throw out the bottom half.
                Console.WriteLine("Trimming queue...");

                // Make buckets for each key count size.
                SearchStateDictionary[] buckets = new SearchStateDictionary[maxKeyCount + 1];
                for (int keyCount = 0; keyCount <= maxKeyCount; keyCount++)
                {
                    buckets[keyCount] = new SearchStateDictionary();
                }

                // Fill the buckets.
                Console.WriteLine("  Filling buckets...");
                while (states.Count > 0)
                {
                    var entry = states.First();
                    states.Remove(entry.Key);
                    buckets[entry.Value.Keys.Count].Add(entry.Key, entry.Value);
                }

                // Move from the buckets into a new queue until a limit is reached.
                Console.WriteLine("  Emptying buckets...");
                var newStates = new SearchStateDictionary();
                while (newStates.Count < MAX_ENTRIES / 4)
                {
                    for (int i = maxKeyCount; i >= 0; i--)
                    {
                        if (buckets[i].Count == 0)
                        {
                            continue;
                        }
                        var entry = buckets[i].First();
                        buckets[i].Remove(entry.Key);
                        newStates.Add(entry.Key, entry.Value);
                    }
                }

                states = newStates;
                Console.WriteLine("  Queue trimmed.");
            }
        }

        // Score possible next steps.
        private bool IsViablePath(SearchState state, char option)
        {
            if (state.Path == "")
            {
                return true;
            }

            char current = state.Path.Last();

            // Don't try to go to where we already are.
            if (option == current)
            {
                return false;
            }
            // Don't try to keep going if we're already finished.
            if (state.Keys.Count >= 26)
            {
                return false;
            }
            // Don't recurse too deeply.
            if (state.Path.Length > 300)
            {
                return false;
            }
            // Don't just move back and forth.
            if (state.Path.Length > 2 && option == state.Path[state.Path.Length - 3])
            {
                return false;
            }
            // Don't go to a lock you don't have the key to.
            if (char.IsUpper(option) && !state.Keys.Contains(Char.ToLower(option)))
            {
                return false;
            }
            // Don't go right back unless you came here to pick up a key.
            bool gotKey = char.IsLower(current) && state.Path.Count(c => c == current) == 1;
            if (!gotKey && state.Path.Length > 1 && option == state.Path[state.Path.Length - 2])
            {
                return false;
            }

            return true;
        }

        private void StepSearch(SearchState state, SearchStateQueue queue)
        {
            // Get the set of every landmark reachable from this spot.
            var reachable = GetReachableLandmarks(state.Path.Last());

            // Sort the options by score.
            var options = from entry in reachable
                          where IsViablePath(state, entry.Key)
                          select entry;

            foreach (var entry in options)
            {
                var current = entry.Key;
                var dist = entry.Value;

                var newState = state;
                newState.Path = state.Path + current;
                newState.Distance = state.Distance + dist;

                // Check the current spot for whether we should update state.
                if (Char.IsLower(current))
                {
                    // We picked up a key maybe.
                    if (!newState.Keys.Contains(current))
                    {
                        newState.Keys = newState.Keys.Add(current);
                    }
                }

                // Okay, this is a valid path. Traverse it.
                queue.Add(newState);
            }
        }

        public void Search()
        {
            var state = new SearchState();
            state.Keys = ImmutableHashSet<char>.Empty;
            state.Distance = 0;
            state.Path = "@";

            var queue = new SearchStateQueue();
            queue.Add(state);

            var bestState = state;

            while (true)
            {
                state = queue.Pop();
                if (state.Keys.Count > bestState.Keys.Count)
                {
                    bestState = state;
                    Console.Write("best={0}\n", bestState);
                }
                else if (state.Keys.Count == bestState.Keys.Count)
                {
                    if (state.Distance < bestState.Distance)
                    {
                        bestState = state;
                        Console.Write("best={0}\n", bestState);
                    }
                }

                //Console.Write("best={0}, current={1}\n", bestState, state);
                StepSearch(state, queue);
            }
        }
    }
}
