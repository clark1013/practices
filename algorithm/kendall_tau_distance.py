# Kendall tau 距离
# 给定两个序列，两个序列的元素相同且没有重复，仅排列不同
# 每当有一对不同的排列时，其距离加一
# 例如 [0,3,1,6,2,5,4] 与 [1,0,3,6,4,2,5] 的距离为 4，不同的排列额分别为 0-1,3-1,2-4,5-4


def distance(slice1, slice2):
    return abs(insert_sort_move_steps(slice1) - insert_sort_move_steps(slice2))


# 将一个序列冒泡排序所需要的移动步数
# 无法解决此问题
def bubble_sort_move_steps(a):
    length = len(a)
    result = 0
    for i in range(length-1, 1, -1):
        for j in range(i):
            if a[j] > a[j+1]:
                a[j], a[j+1] = a[j+1], a[j]
                result += 1
                print(a)
    print(result)
    return result

def insert_sort_move_steps(a):
    length = len(a)
    result = 0
    for i in range(1, length):
        for j in range(i, 0, -1):
            if a[j] < a[j-1]:
                a[j], a[j-1] = a[j-1], a[j]
                result += 1
                print(a)
    print(result)
    return result


print(distance([0,3,1,6,2,5,4], [1,0,3,6,4,2,5]))
