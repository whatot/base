#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Debug, Clone)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode {
            val: val,
            next: None,
        }
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { top: None }
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TestDataNode {
    a: i32,
}

#[test]
fn test_stack_operation() {
    let a = TestDataNode { a: 5 };
    let b = TestDataNode { a: 9 };

    let mut s = Stack::<&TestDataNode>::new();
    assert_eq!(None, s.pop());

    s.push(&a);
    s.push(&b);
    assert_eq!(Some(&b), s.pop());
    assert_eq!(Some(&a), s.pop());
    assert_eq!(None, s.pop());
}
