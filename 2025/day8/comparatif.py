from math import inf


class DSU:
    def __init__(self, n):
        self.parent = list(range(n))
        self.size = [1] * n
        self.components = n


    def find(self, x):
        while self.parent[x] != x:
            self.parent[x] = self.parent[self.parent[x]]
            x = self.parent[x]
        return x


    def union(self, a, b):
        ra = self.find(a)
        rb = self.find(b)
        if ra == rb:
            return False
        if self.size[ra] < self.size[rb]:
            ra, rb = rb, ra
        self.parent[rb] = ra
        self.size[ra] += self.size[rb]
        self.components -= 1
        return True



def main():
    points = []
    with open("day8/data/input.txt", "r") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            x, y, z = map(int, line.split(","))
            points.append((x, y, z))


    n = len(points)
    edges = []
    for i in range(n):
        x1, y1, z1 = points[i]
        for j in range(i + 1, n):
            x2, y2, z2 = points[j]
            dx = x1 - x2
            dy = y1 - y2
            dz = z1 - z2
            d2 = dx*dx + dy*dy + dz*dz
            edges.append((d2, i, j))
    edges.sort(key=lambda e: e[0])
    dsu = DSU(n)
    connections = 0
    part1 = None
    part2 = None
    for d2, i, j in edges:
        if connections == 1000 and part1 is None:
            comp_sizes = {}
            for v in range(n):
                root = dsu.find(v)
                comp_sizes[root] = comp_sizes.get(root, 0) + 1
            sizes = sorted(comp_sizes.values(), reverse=True)
            while len(sizes) < 3:
                sizes.append(1)
            print(sizes[0], sizes[1], sizes[2])
            part1 = sizes[0] * sizes[1] * sizes[2]
        merged = dsu.union(i, j)
        if merged and dsu.components == 1:
            x1, _, _ = points[i]
            x2, _, _ = points[j]
            part2 = x1 * x2
            break


        connections += 1
    if part1 is None:
        comp_sizes = {}
        for v in range(n):
            root = dsu.find(v)
            comp_sizes[root] = comp_sizes.get(root, 0) + 1
        sizes = sorted(comp_sizes.values(), reverse=True)
        while len(sizes) < 3:
            sizes.append(1)
        part1 = sizes[0] * sizes[1] * sizes[2]


    print("Part 1:", part1)
    print("Part 2:", part2)



if __name__ == "__main__":
    main()
