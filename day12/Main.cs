using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Advent2021Day12
{
    class Advent
    {
        public static void Main()
        {
            var graph = new Graph();
            foreach (string line in File.ReadLines(@"C:\Users\emmga\source\repos\Advent2021Day12\Advent2021Day12\input12.txt"))
            {
                graph.ParseLine(line);
            }
            Tree pathTree = Tree.PathTree(graph);
            int part1 = pathTree.CountEndLeaves();
            Tree part2Tree = Tree.PathTreeP2(graph);
            int part2 = part2Tree.CountEndLeaves();
            Console.WriteLine($"Day 12 Part 1: {part1}");
            Console.WriteLine($"Day 12 Part 2: {part2}");
            Console.WriteLine("Press any key to quit...");
            Console.ReadKey();
        }
    }
}
