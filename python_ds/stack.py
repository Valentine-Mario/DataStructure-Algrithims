class Empty(Exception):
    """Error attempting to access an element from an empty container."""
    pass


class ArrayStack:
    """LIFO Stack implementation using a Python list as underlying storage."""

    def __init__(self):
        self.data =[]

    def __len__(self):
        return len(self.data)

    def is_empty(self):
        return len(self.data) == 0

    def push(self, item):
        self.data.append(item)

    def top(self):
        if self.is_empty():
            raise Empty("No item on the stack")
        return self.data[-1]

    def pop(self):
        if self.is_empty():
            raise Empty("No item on the stack")
        return self.data.pop()


s=ArrayStack()
s.push(28)
s.push("hello")
s.top()
print(s.pop())

#using stack to determine correct string sequence
def is_matched(expr):
    left='({['
    right=')}]'
    s=ArrayStack()

    for i in expr:
        if i in left:
            #push the opening bracket on the stack
            s.push(i)
        elif i in right:
            #if char is closing bracket and there is no opening bracket on the Stack
            #return false
            if s.is_empty():
                return False
            #if the char index of the closing bracket is not in the
            #similar to it's closing ratner, throw False
            #else pop item from stack
            if right.index(i)!=left.index(s.pop()):
                return False
    #if stack is empty at the end of the iteration, item matches
    return s.is_empty()

print(is_matched("(){}["))