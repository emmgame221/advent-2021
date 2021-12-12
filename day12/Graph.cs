using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Advent2021Day12
{
    public class Graph
    {
        public List<GraphNode> Nodes { get; private set; }
        public GraphNode Start { 
            get
            {
                return this.Nodes.Find((GraphNode n) => n.NType == NodeType.Start);
            }
        }

        public Graph()
        {
            this.Nodes = new List<GraphNode>();
        }

        public void ParseLine(string line)
        {
            var parts = line.Split('-');
            string u = parts[0];
            string v = parts[1];
            GraphNode uNode = this.Nodes.Find((GraphNode n) => n.Name.Equals(u));
            GraphNode vNode = this.Nodes.Find((GraphNode n) => n.Name.Equals(v));
            if (uNode == null) {
                uNode = new GraphNode(u);
                this.Nodes.Add(uNode);
            }
            if (vNode == null)
            {
                vNode = new GraphNode(v);
                this.Nodes.Add(vNode);
            }
            uNode.AddEdge(vNode);
            vNode.AddEdge(uNode);
        }

        public override string ToString()
        {
            var builder = new StringBuilder();
            foreach (var n in this.Nodes)
            {
                builder.Append(n);
                builder.AppendLine();
            }
            return builder.ToString();
        }
    }

    public class GraphNode
    {
        public string Name { get; private set; }
        public List<GraphNode> Edges { get; private set; }
        public NodeType NType { get; private set; }

        public GraphNode(string name)
        {
            this.Name = name;
            this.Edges = new List<GraphNode>();
            if (name == "start")
            {
                this.NType = NodeType.Start;
            } else if (name == "end")
            {
                this.NType = NodeType.End;
            } else if (name.ToUpper() == name)
            {
                this.NType = NodeType.Big;
            } else
            {
                this.NType = NodeType.Small;
            }
        }

        public void AddEdge(GraphNode v)
        {
            this.Edges.Add(v);
        }

        public override bool Equals(object obj)
        {
            if (obj is GraphNode)
            {
                var other = (GraphNode)obj;
                return other.Name.Equals(this.Name);
            }
            else 
            { 
                return base.Equals(obj); 
            }
        }

        public override int GetHashCode()
        {
            return base.GetHashCode();
        }

        public override string ToString()
        {
            var builder = new StringBuilder();
            foreach (var v in this.Edges) {
                builder.Append(this.Name);
                builder.Append("-");
                builder.Append(v.Name);
                builder.AppendLine();
            }
            return builder.ToString();
        }
    }

    public enum NodeType
    {
        Start,
        Big,
        Small,
        End
    }

}
