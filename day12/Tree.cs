using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Advent2021Day12
{
    public class Tree
    {
        public TreeNode Root { get; private set; }

        public static Tree PathTree(Graph graph)
        {
            Tree tree = new Tree();
            tree.Root = new TreeNode("start", null, NodeType.Start, false);
            List<GraphNode> graphStack = new List<GraphNode>();
            List<TreeNode> treeStack = new List<TreeNode>();
            graphStack.Add(graph.Start);
            treeStack.Add(tree.Root);
            while(!(graphStack.Count==0) && !(treeStack.Count==0))
            {
                GraphNode gn = graphStack[graphStack.Count - 1];
                graphStack.RemoveAt(graphStack.Count - 1);
                TreeNode tn = treeStack[treeStack.Count - 1];
                treeStack.RemoveAt(treeStack.Count - 1);
                foreach (var v in gn.Edges)
                {
                    if (v.NType == NodeType.Big)
                    {
                        TreeNode child = new TreeNode(v.Name, tn, NodeType.Big, false);
                        tn.AddChild(child);
                        graphStack.Add(v);
                        treeStack.Add(child);
                    } else if (v.NType == NodeType.Small)
                    {
                        if (!(tn.DescendsFrom(v.Name)))
                        {
                            TreeNode child = new TreeNode(v.Name, tn, NodeType.Small, false);
                            tn.AddChild(child);
                            graphStack.Add(v);
                            treeStack.Add(child);
                        }
                    } else if (v.NType == NodeType.End)
                    {
                        TreeNode child = new TreeNode(v.Name, tn, NodeType.End, false);
                        tn.AddChild(child);
                    }
                }
            }
            return tree;
        }

        public static Tree PathTreeP2(Graph graph)
        {
            Tree tree = new Tree();
            tree.Root = new TreeNode("start", null, NodeType.Start, true);
            List<GraphNode> graphStack = new List<GraphNode>();
            List<TreeNode> treeStack = new List<TreeNode>();
            graphStack.Add(graph.Start);
            treeStack.Add(tree.Root);
            while (!(graphStack.Count == 0) && !(treeStack.Count == 0))
            {
                GraphNode gn = graphStack[graphStack.Count - 1];
                graphStack.RemoveAt(graphStack.Count - 1);
                TreeNode tn = treeStack[treeStack.Count - 1];
                treeStack.RemoveAt(treeStack.Count - 1);
                foreach (var v in gn.Edges)
                {
                    if (v.NType == NodeType.Big)
                    {
                        TreeNode child = new TreeNode(v.Name, tn, NodeType.Big, tn.DoubleSmallAllowed);
                        tn.AddChild(child);
                        graphStack.Add(v);
                        treeStack.Add(child);
                    }
                    else if (v.NType == NodeType.Small)
                    {
                        if (!(tn.DescendsFrom(v.Name)))
                        {
                            TreeNode child = new TreeNode(v.Name, tn, NodeType.Small, tn.DoubleSmallAllowed);
                            tn.AddChild(child);
                            graphStack.Add(v);
                            treeStack.Add(child);
                        } else if (tn.DoubleSmallAllowed)
                        {
                            TreeNode child = new TreeNode(v.Name, tn, NodeType.Small, false);
                            tn.AddChild(child);
                            graphStack.Add(v);
                            treeStack.Add(child);
                        }
                    }
                    else if (v.NType == NodeType.End)
                    {
                        TreeNode child = new TreeNode(v.Name, tn, NodeType.End, tn.DoubleSmallAllowed);
                        tn.AddChild(child);
                    }
                }
            }
            return tree;
        }

        public int CountEndLeaves()
        {
            return Root.CountEndLeaves();
        }
    }

    public class TreeNode
    {
        public TreeNode Parent { get; private set; }
        public string Name { get; private set; }
        public NodeType NType { get; private set; }
        public List<TreeNode> Children { get; private set; }
        public bool DoubleSmallAllowed { get; private set; }

        public TreeNode(string name, TreeNode parent, NodeType ntype, bool allowDoubleSmall)
        {
            this.Name = name;
            this.Parent = parent;
            this.NType = ntype;
            this.Children = new List<TreeNode>();
            this.DoubleSmallAllowed = allowDoubleSmall;
        }

        public bool DescendsFrom(string name)
        {
            bool result = false;
            TreeNode pointer = this.Parent;
            while (pointer != null) {
                if (pointer.Name == name) {
                    result = true;
                    break;
                }
                pointer = pointer.Parent;
            }
            return result;
        }

        public void AddChild(TreeNode node)
        {
            if (!(this.Children.Contains(node)))
            {
                this.Children.Add(node);
            }
        }

        public int CountEndLeaves()
        {
            if (this.NType == NodeType.End)
            {
                return 1;
            } else
            {
                return this.Children.Sum((n) => n.CountEndLeaves());
            }
        }

        public override bool Equals(object obj)
        {
            if (obj is TreeNode)
            {
                var other = (TreeNode)obj;
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
    }
}
