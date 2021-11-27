# Kendall tau 距离
# 给定两个序列，两个序列的元素相同且没有重复，仅排列不同
# 每当有一对不同的排列时，其距离加一
# 例如 [0,3,1,6,2,5,4] 与 [1,0,3,6,4,2,5] 的距离为 4，不同的排列额分别为 0-1,3-1,2-4,5-4
import copy


# n平方时间复杂度算法
def distance_n2(slice1, slice2):
    sa = insert_sort_exchanges(slice1)
    sb = insert_sort_exchanges(slice2)
    return len(sa - sb) + len(sb - sa)


# 插入排序所有交换的集合
def insert_sort_exchanges(a):
    length = len(a)
    result = set()
    for i in range(1, length):
        for j in range(i, 0, -1):
            if a[j] < a[j - 1]:
                a[j], a[j - 1] = a[j - 1], a[j]
                result.add((a[j], a[j - 1]))
    return result


def distance(slice1, slice2):
    index_map = {}
    for i, v in enumerate(slice1):
        index_map[v] = i

    indexes = []
    for v in slice2:
        indexes.append(index_map[v])

    # 如果用归并其时间复杂度会更低
    return bubble_sort_move_steps(indexes)


# 将一个序列冒泡排序所需要的移动步数
def bubble_sort_move_steps(a):
    length = len(a)
    result = 0
    for i in range(length - 1, 1, -1):
        for j in range(i):
            if a[j] > a[j + 1]:
                a[j], a[j + 1] = a[j + 1], a[j]
                result += 1
    return result


if __name__ == "__main__":
    A = [0, 3, 1, 6, 2, 5, 4]
    B = [1, 0, 3, 6, 4, 2, 5]
    print(distance_n2(copy.deepcopy(A), copy.deepcopy(B)))
    print(distance(copy.deepcopy(A), copy.deepcopy(B)))
