using System;

namespace code
{
    class Program
    {
        static void Main(string[] args)
        {
            var map = Map.ParseMap("../input.txt");
            Console.WriteLine("Computing landmarks...");
            map.ComputeLandmarks();
            Console.WriteLine(map.ToString());
            map.Search();
        }
    }
}