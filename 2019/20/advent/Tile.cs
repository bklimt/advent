using System;
using System.Collections.Generic;

namespace advent
{
    public class Tile
    {
        public enum TileType { Unknown, Empty, Wall, Label, Portal }

        public TileType Type { get; set; }
        public char Value { get; set; }
        public string PortalName { get; set; }

        // The distance through the maze to portal.
        public Dictionary<string, int> landmarkDists { get; private set; } =
            new Dictionary<string, int>();

        public char ToChar()
        {
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
        public bool UpdateLandmarks(Tile neighbor)
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

            if (neighbor.Type == TileType.Portal)
            {
                // The neighbor is impassable, so don't inherit distances.
                // But it is a landmark, so record it.
                if (!landmarkDists.ContainsKey(neighbor.PortalName))
                {
                    landmarkDists[neighbor.PortalName] = 0;
                    return true;
                }
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