using System;

namespace advent
{
    class Program
    {
        static void Main(string[] args)
        {
            var map = Map.ParseMap("test2.txt");
            map.ComputePortals();
            Console.WriteLine(map.ToString());

            Console.WriteLine("Initial portals:");
            map.PrintPortals();

            Console.WriteLine("Full portals:");
            map.ComputeLandmarks();
            map.PrintPortals();
            Console.WriteLine(map.ToString());
        }
    }
}

// 1: 514
