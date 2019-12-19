using System;
using System.Collections.Generic;
using System.IO;

namespace code
{
    public class Tile
    {
        public enum TileType { Unknown, Empty, Wall, Key, Lock }

        public TileType Type { get; set; }
        public char Value { get; set; }

        // The distance through the maze to every key and lock (and the start).
        public Dictionary<char, int> landmarkDists { get; private set; } =
            new Dictionary<char, int>();

        public char ToChar()
        {
            if (Type == TileType.Lock)
            {
                return Char.ToUpper(Value);
            }
            if (Type == TileType.Key)
            {
                return Value;
            }
            return Type.ToChar();
        }

        // Updates the internal bookkeeping of how far all the landmarks are.
        public bool UpdateLandmarks(Tile neighbor)
        {
            // Impassable tiles don't need to know distances.
            if (Type == TileType.Wall || Type == TileType.Unknown)
            {
                return false;
            }

            // It matters what type this is and what type its neighbor is.
            if (neighbor.Type == TileType.Unknown || neighbor.Type == TileType.Wall)
            {
                // The neighbor is impassable, and not a landmark.
                return false;
            }

            if (neighbor.Type == TileType.Lock || neighbor.Type == TileType.Key)
            {
                // The neighbor is impassable, so don't inherit distances.
                // But it is a landmark, so record it.
                if (!landmarkDists.ContainsKey(neighbor.ToChar()))
                {
                    landmarkDists[neighbor.ToChar()] = 1;
                    return true;
                }
                return false;
            }

            var updated = false;

            // First, make sure this tile knows itself.
            if (Type == TileType.Key || Type == TileType.Lock)
            {
                if (!landmarkDists.ContainsKey(ToChar()))
                {
                    landmarkDists[ToChar()] = 0;
                    updated = true;
                }
            }

            // Now inherit distances from the neighbor.
            foreach (var pair in neighbor.landmarkDists)
            {
                if (!landmarkDists.ContainsKey(pair.Key))
                {
                    landmarkDists[pair.Key] = pair.Value + 1;
                    updated = true;
                    continue;
                }
                if (pair.Value + 1 < landmarkDists[pair.Key])
                {
                    landmarkDists[pair.Key] = pair.Value + 1;
                    updated = true;
                    continue;
                }
            }
            return updated;
        }
    }
}