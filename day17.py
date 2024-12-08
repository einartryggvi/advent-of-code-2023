import heapq

def next_directions(current_direction):
    if current_direction == (-1, 0):
        return [(-1, 0), (0,1), (0,-1)]
    elif current_direction == (1, 0):
        return [(1, 0), (0, 1), (0, -1)]
    elif current_direction == (0, -1):
        return [(0, -1), (1,0), (-1, 0)]
    elif current_direction == (0, 1):
        return [(0, 1), (1, 0), (-1, 0)]
    else:
      return [(-1, 0), (1, 0), (0, -1), (0, 1)] 


def min_cost_path(matrix, start_point, start_direction, target_point):
    queue = [(0, start_point, start_direction, 0)]
    visited = {}

    while queue:
        cost, point , direction, tracker = heapq.heappop(queue)

        if point == target_point:
            return cost

        visited_state = (point, direction, tracker)
        if visited_state in visited and visited[visited_state] <= cost:
            continue
        visited[visited_state] = cost
        
        for next_direction in next_directions(direction):
            next_point = (point[0] + next_direction[0], point[1] + next_direction[1])
            next_tracker = tracker + 1 if direction == next_direction else 0
            if next_point in matrix and next_tracker < 3:
                queue.append((cost + int(matrix[next_point]), next_point, next_direction, next_tracker))

    return 999999999

def min_cost_path2(matrix, start_point, start_direction, target_point):
    queue = [(0, start_point, start_direction, 0)]
    visited = {}

    while queue:
        cost, point , direction, tracker = heapq.heappop(queue)

        if point == target_point and tracker >= 4:
            return cost

        visited_state = (point, direction, tracker)
        if visited_state in visited and visited[visited_state] <= cost:
            continue
        visited[visited_state] = cost
        
        for next_direction in next_directions(direction):
            next_point = (point[0] + next_direction[0], point[1] + next_direction[1])
            if next_point in matrix:
              if next_direction == direction:
                if tracker < 10:
                  queue.append((cost + int(matrix[next_point]), next_point, next_direction, tracker + 1))
              else:
                if tracker >= 4:
                  queue.append((cost + int(matrix[next_point]), next_point, next_direction, 1))

    return 999999999

def part1():
    maxI = 0
    maxJ = 0
    with open('./inputs/day17.txt') as file:
        matrix = {}
        for i, line in enumerate(file):
            maxI = i
            for j, char in enumerate(line.strip()):
                maxJ = j
                matrix[i, j] = char

    return min_cost_path(matrix, (0,0), (1, 0), (maxI, maxJ))

def part2():
    maxI = 0
    maxJ = 0
    with open('./inputs/day17.txt') as file:
        matrix = {}
        for i, line in enumerate(file):
            maxI = i
            for j, char in enumerate(line.strip()):
                maxJ = j
                matrix[i, j] = char

    return min_cost_path2(matrix, (0,0), (1, 0), (maxI, maxJ))


print("Part 1: ", part1())
print("Part 2: ", part2())
