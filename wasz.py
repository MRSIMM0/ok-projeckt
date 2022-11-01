from math import sqrt
import time
import sys
sys.setrecursionlimit(2_000_000)

def main():
    file = open("./data.txt")
    
    data = handle_file(file)

    heuristic(data)
    
def heuristic(data):
    
    visited = []

    current = 0
    test = time.time()
    recur(data,visited, current)
    print(time.time() - test)
    print(visited)

    pass

def recur(arr,visited,current):

    visited.append(current)

    if(len(arr) == len(visited)):
        return
    
    distance = [calculate_data(x,arr[current]) for x in arr]

    sort = sorted(distance)

    min = 0

    while  distance.index(sort[min]) in visited: 
        min += 1
        pass
    
    current = distance.index(sort[min])

    recur (arr,visited,current)
    pass

def handle_file(file):

    result = []

    for line in file:
        try:
            x,y = line.replace("\n","").split(" ")
            result.append((int(x),int(y)))
        except:
            pass
    return result

def calculate_data(v1,v2):
    return sqrt(pow((v1[0]-v2[0]),2)+ pow((v1[1]-v2[1]),2))

main()


