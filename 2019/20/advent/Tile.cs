using System;
using System.Collections.Generic;

namespace advent
{
    public class Tile
    {
        public enum TileType { Unknown, Empty, Wall, Label, Portal }

        public enum Direction { Unknown, Up, Down, Left, Right }

        public TileType Type { get; set; }
        public char Value { get; set; }
        public string PortalName { get; set; }
        public Direction ZDirection { get; set; }

        // The distance through the maze to portal.
        public Dictionary<string, int> landmarkDists { get; private set; } =
            new Dictionary<string, int>();

        public char ToChar()
        {
            if (ZDirection != Direction.Unknown)
            {
                switch (ZDirection)
                {
                    case Direction.Down: return 'v';
                    case Direction.Up: return '^';
                    case Direction.Left: return '<';
                    case Direction.Right: return '>';
                }
            }
            if (Type == TileType.Label)
            {
                return Char.ToLower(Value);
            }
            if (Type == TileType.Portal)
            {
                return Value;
            }
            return Type.ToChar();
        }

        public bool MergeLandmarks(Tile other)
        {
            bool updated = false;
            if (Type != TileType.Portal || other.Type != TileType.Portal)
            {
                throw new Exception("Tried to merge non-portals.");
            }
            foreach (var entry in other.landmarkDists)
            {
                if (!landmarkDists.ContainsKey(entry.Key) || entry.Value < landmarkDists[entry.Key])
                {
                    landmarkDists[entry.Key] = entry.Value;
                    updated = true;
                }
            }
            return updated;
        }

        // Updates the internal bookkeeping of how far all the landmarks are.
        public bool UpdateLandmarks(Tile neighbor, Direction dir)
        {
            // Impassable tiles don't need to know distances.
            if (Type == TileType.Wall || Type == TileType.Unknown || Type == TileType.Label)
            {
                return false;
            }

            // It matters what type this is and what type its neighbor is.
            if (neighbor.Type == TileType.Unknown || neighbor.Type == TileType.Wall)
            {
                // The neighbor is impassable, and not a landmark.
                return false;
            }

            var updated = false;

            // First, make sure this tile knows itself.
            if (Type == TileType.Portal)
            {
                if (!landmarkDists.ContainsKey(PortalName))
                {
                    landmarkDists[PortalName] = 0;
                    updated = true;
                }
            }

            // Now inherit distances from the neighbor.
            foreach (var pair in neighbor.landmarkDists)
            {
                int delta = 1;
                // It's free to jump to portals.
                if (neighbor.Type == TileType.Portal)
                {
                    delta = 0;
                }

                if (!landmarkDists.ContainsKey(pair.Key) || (pair.Value + delta < landmarkDists[pair.Key]))
                {
                    landmarkDists[pair.Key] = pair.Value + delta;
                    updated = true;
                    if (pair.Key == "ZZ")
                    {
                        ZDirection = dir;
                    }
                }
            }
            return updated;
        }
    }
}