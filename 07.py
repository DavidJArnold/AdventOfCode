with open("07.real.txt", "r") as file:
    cmds = file.read().split("\n")


class node:
    """
    A node class to be used with the tree class below. A node represents a directory.
    """

    def __init__(self, tree, name, depth, is_root: bool = False, parent=None):
        self.id = tree.get_new_node_id()  # unique identifier
        self.file_sizes = 0  # sum of file sizes at this level of the directory
        self.name = name  # the name of this directory
        self.depth = depth  # depth from home directory (= 0)
        self.parent = parent  # the parent node (the directory containing this one (only one level up))
        self.children = (
            []
        )  # the children of this node (the directories contained in this one (only one level down))
        self.root = is_root  # indicator for the root node (the home directory)

        # add the new node to its parent's list of children
        for node in tree.nodes:
            if node == parent:
                node.children.append(self)

        def __eq__(self, other):
            # so we can test for node equlity
            return self.id == other.id


class tree:
    """
    A tree is just a collection of nodes
    """

    def __init__(self):
        self.nodes = []

    def get_new_node_id(self):
        return len(self.nodes) + 1

    def get_size(self, node):
        # recursively find the size of the current directory and it's sub-directories
        return node.file_sizes + sum([self.get_size(n) for n in node.children])


depth = 0  # track depth -- if it decreases, we are exiting a subdirectory
files = tree()  # the tree that tracks our directories
# recent_nodes keeps a list of recently created trees at each depth,
# so we know which node to set as the parent when creating a new node
recent_nodes = {}
for line in cmds:
    # step through the input file
    # there are some assumptions in this, i.e. we only do ls in a directory once, etc.
    if len(line) == 0:
        # skip empty lines
        continue
    elif line == "$ cd /":
        # this is the first line in the file
        # create the node, add it to the files tree, and record it as the only node at depth 0
        new_node = node(files, line.split(" ", 1)[1], depth, True, None)
        files.nodes.append(new_node)
        recent_nodes[depth] = new_node
    elif line[:4] == "$ cd" and line[5:7] != "..":
        # moving to a new subdirectory
        depth += 1  # depth increases
        # the new node will have a name from the cd command, and it's
        # parent will be the most recent node created at the previous depth
        # This is because we don't skip back and forth between different
        # branches of the file system
        new_node = node(
            files, line.split(" ", 1)[1], depth, False, recent_nodes[depth - 1]
        )
        files.nodes.append(new_node)
        recent_nodes[depth] = new_node
    elif line[:7] == "$ cd ..":
        # going back up a level in the file system
        depth -= 1
    elif line[0] != "$" and line[0] != "d":
        # it's a file, we just add the size to the sum for that directory
        new_node.file_sizes += int(line.split(" ", 1)[0])

# all_sizes is the total size of each directory including subdirectories
all_sizes = [files.get_size(n) for n in files.nodes]

# get the sum of directories under 100000 in size
print("Part 1: ", sum([s for s in all_sizes if s < 100000]))

# We need 70M - used_space  > 30M, that is, 40M > used_space
# used_space = total_used_space - size_of_deleted_subdirectory
# so we want the smallest size_of_deleted_subdirectory such that
# total_used_space - size_of_deleted_subdirectory < 40M
# or size_of_deleted_subdirectory + 40M > total_used_space
need_to_clear = 40000000
print(
    "Part 2: ",
    min([t for t in all_sizes if all_sizes[0] - t < need_to_clear]),
)
