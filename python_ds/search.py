val=[2, 3, 1, 10, 11, 9, 23, 12, 11, 23, 90, 19, 18, 900, 12, 29, 10]

sorted_val= sorted(val)

def binary_search(data, target, low, high):
    if low>high:
        return False
    else:
        mid=(low + high)//2
        if target==data[mid]:
            return True
        elif target<data[mid]:
            return binary_search(data, target, low, mid-1)
        else:
            return binary_search(data, target, mid+1, high)



k= binary_search(sorted_val, 2, 0, len(sorted_val)-1)
print(k)


def power(x, n):
    if n==0:
        return 1
    else:
        return x*power(x, n-1)
    

print(power(23, 6))



def poweroptimized(x, n):
    if n==0:
        return 1
    else:
        val=poweroptimized(x, n//2)
        result=val*val
        #if value is odd eg 2^9=2.2^4.2^4
        if n % 2==1:
            result*=x
        return result

print(poweroptimized(2, 8))


def bin_sum(list, start, stop):
    if start>stop:
        return 0
    elif start==stop-1:
        return list[start]
    else:
        mid=(start+stop)//2
        return bin_sum(list, start, mid)+bin_sum(list, mid, stop)
        

print(bin_sum([1, 2, 3], 0, 3))

def minimum(l, val):
    if val==1:
        return l[0]
    
    return min(l[val-1], minimum(l, val-1))

l = [34, 23, 2, 4, 18]
print(minimum(l, len(l)))

def maxinum(l, val):
    if val==1:
        return l[0]
    
    return max(l[val-1], maxinum(l, val-1))

l = [34, 23, 2, 4, 18]
print(maxinum(l, len(l)))

