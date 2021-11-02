class ArrayStack {
  constructor(maxlength) {
    this.data = [];
    this.maxlength = maxlength;
  }

  len() {
    return this.data.length;
  }

  is_empty() {
    return this.data.length === 0;
  }

  push(item) {
    if (this.data.length === this.maxlength) {
      console.error(
        "stack already at max length. Consider expanding stack size"
      );
    } else {
      this.data.push(item);
    }
  }

  expand(num) {
    this.maxlength += num;
  }

  top() {
    if (this.data.length === 0) {
      console.error("stack currently empty. Nothing to see");
    } else {
      return this.data[this.data.length - 1];
    }
  }

  pop() {
    if (this.data.length === 0) {
      console.error("stack currently empty. Nothing to pop");
    } else {
      this.maxlength -= 1;
      return this.data.pop();
    }
  }
}


var new_stack= new ArrayStack(1)
new_stack.push(1);
new_stack.push(2);
new_stack.expand(3);
new_stack.push(3);
console.log(new_stack.len())