use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// destructor for LinkedList
impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = curr_link {
            curr_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(10);
        list.push(20);
        list.push(30);

        assert_eq!(list.pop(), Some(30));
        assert_eq!(list.pop(), Some(20));

        list.push(50);
        list.push(40);

        assert_eq!(list.pop(), Some(40));
        assert_eq!(list.pop(), Some(50));

        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), None);
    }
}
