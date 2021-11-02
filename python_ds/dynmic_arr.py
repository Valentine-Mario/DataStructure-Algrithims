import ctypes


class Dynamic_array:
    def __init__(self):
        self._n=0
        self._capacity=1
        self._A=self._make_array(self._capacity)

    def __len__(self):
        return self._n

    def __getitem__(self, k):
        if not 0 <= k < self._n:
            raise IndexError("invalid index")
        return self._A[k]

    def append(self, obj):
        if self._n==self._capacity:
            self._resize(2*self._capacity)
        self._A[self._n]=obj
        self._n+=1

    def _resize(self, c):
        #increase size of A and reassign it to a larger array B
        B=self._make_array(c)
        for k in range(self._n):
            B[k]=self._A[k]
        self._A=B
        self._capacity=c

    def _make_array(self, c):
        return (c * ctypes.py_object)()


k=Dynamic_array()
k.append(1)
k.append(5)
k.append(11)
k.append(34)

for item in range(k.__len__()):
    print(k.__getitem__(item))
