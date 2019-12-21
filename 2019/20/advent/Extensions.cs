using System;

namespace advent
{
    public static class Extensions
    {
        public static char ToChar(this Tile.TileType type)
        {
            switch (type)
            {
                case Tile.TileType.Unknown: return ' ';
                case Tile.TileType.Empty: return '.';
                case Tile.TileType.Wall: return '#';
                case Tile.TileType.Portal: return '&';
            }
            throw new Exception("unknown type");
        }
    }
}