using System;

namespace code
{
    class Program
    {
        static void Main(string[] args)
        {
            var map = Map.ParseMap(args[0]);
            Console.WriteLine("Computing landmarks...");
            map.ComputeLandmarks();
            Console.WriteLine(map.ToString());
            map.Search();
        }
    }
}

//
// best=keys=19, dist=3494,
// path=@ogoadiOGjGODzJqmqJzDouoDzJqmUbxbUmqJzD@ndMeMdiNpkpNi@ouBlslBuoDzJqmUbSy
//
// best=keys=19, dist=3288,
// path=@aiogugodn@DzDOGjGODzJqmUbxbUmqJzDiNpkpNidMeMdouBlslBuoDzJqmUbSy
//
// best=keys=21, dist=3882,
// path=@aiogugodn@DzDOGjGODzJqmUbxbUmqJzDiNpkpNidMeMdouBlslBuoDzJqmUbSySbUmqJzD@nKYhc
//
// best=keys=26, dist=4618,
// path=@aiogugodn@DzDOGjGODzJqmUbxbUmqJzDiNpkpNidMeMdouBlslBuoDzJqmUbSySbUmqJzD@nKYhchYKn@DzJqmUbSyHCrRvRrVwtf
//