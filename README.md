# Implement a Queue using two Stacks

This is a learning exercise. One stack is used for inserting and the other one for popping.

## Critical Details:

* When a pop is invoked, the element is popped from the out stack.
* If the out stack is empty, then we pop all the elements from the in stack into the out stack,
then we can pop from the out stack.
