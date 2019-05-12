use std::boxed::Box;
use std::convert::{ From, Into };

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value: value,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    
    pub fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    //O(n)
    pub fn push(&mut self, value: T) {
        
        let node = Box::new(Node::new(value));
        if self.head.is_none() {
            self.head = Some(node);
            return;
        }

        let mut p = self.head.as_mut().unwrap();
        loop {
            if p.next.is_none() {
                break;
            }
            p = p.next.as_mut().unwrap();
        }

        p.next = Some(node);
    }

    //O(n)
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let mut holder = &mut self.head;
        loop {
            if holder.as_ref().unwrap().next.is_none() {
                let Node { value, ..} = *holder.take().unwrap();
                return Some(value);
            } else {
                holder = &mut holder.as_mut().unwrap().next;
            }
        }
    }

    //O(1)
    pub fn unshift(&mut self, value: T) {
        let mut node = Box::new(Node::new(value));
        node.next = self.head.take();
        self.head.replace(node);
    }

    //O(1)
    pub fn shift(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let Node { value, next } = *node;
            self.head = next;
            value
        })
    }

    pub fn iter_node<'a>(&'a self) -> LinkedListNodeIterator<'a, T> {
        LinkedListNodeIterator {
            node: &self.head,
        }
    }

    pub fn iter_node_mut<'a>(&'a mut self) -> LinkedListNodeIteratorMut<'a, T> {
        LinkedListNodeIteratorMut {
            node: &mut self.head,
        }
    }
    
    pub fn iter<'a>(&'a self) -> LinkedListValueIterator<'a, T> {
        LinkedListValueIterator(self.iter_node())
    }

    pub fn iter_mut<'a>(&'a mut self) -> LinkedListValueIteratorMut<'a, T> {
        LinkedListValueIteratorMut(self.iter_node_mut())
    }
}

impl<T> std::iter::FromIterator<T> for LinkedList<T> {
    fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item = T> {
        let mut result = LinkedList::new();
        for i in iter {
            result.push(i);
        }
        result
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> LinkedList<T> {
        vec.into_iter().collect()
    }
}

impl<T> Into<Vec<T>> for LinkedList<T> {
    fn into(self) -> Vec<T> {
        self.into_iter().collect::<Vec<_>>()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList [")?;
        for (i, n) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", n)?;
        }
        write!(f, "]")
    }
}

pub struct LinkedListNodeIterator<'a, T> {
    node: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListNodeIterator<'a, T> {
    type Item = &'a Box<Node<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_none() {
            return None;
        }
        let result = self.node.as_ref();
        self.node = &self.node.as_ref().unwrap().next;
        return result;
    }
}

pub struct LinkedListNodeIteratorMut<'a, T> {
    node: &'a mut Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListNodeIteratorMut<'a, T> {
    type Item = &'a mut Box<Node<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_none() {
            return None;
        }
        let result = self.node.as_mut().map(|node| {
            unsafe { &mut *(node as *mut _) }
        });
        self.node = &mut self.node.as_mut().map(|node| {
            unsafe { &mut *(node as *mut Box<Node<T>>) }
        }).unwrap().next;
        return result;
    }
}

pub struct LinkedListIntoValueIterator<T> {
    node: Option<Box<Node<T>>>,
}

impl<T> Iterator for LinkedListIntoValueIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            let Node { value, next } = *node;
            self.node = next;
            value
        })
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIntoValueIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIntoValueIterator {
            node: self.head,
        }
    }
}

pub struct LinkedListValueIterator<'a, T>(LinkedListNodeIterator<'a, T>);

impl<'a, T> Iterator for LinkedListValueIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.0.next().map(|node| &node.value)
    }
}

pub struct LinkedListValueIteratorMut<'a, T>(LinkedListNodeIteratorMut<'a, T>);

impl<'a, T> Iterator for LinkedListValueIteratorMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        self.0.next().map(|node| &mut node.value)
    }
}

#[test]
fn linked_list() {
    let mut list: LinkedList<u8> = vec![1, 2, 3, 4, 5].into();
    for node in list.iter_node_mut() {
        node.value = node.value * 2;
    }
    for node in list.iter_mut() {
        *node = *node * *node;
    }

    println!("{:?}", list);
    list.push(5);
    list.push(6);
    list.push(7);
    println!("{:?}", list);
    println!("list.pop() {:?}", list.pop());
    println!("list.pop() {:?}", list.pop());
    println!("list.pop() {:?}", list.pop());
    list.unshift(8);
    list.unshift(9);
    list.unshift(10);
    println!("{:?}", list);
    println!("list.shift() {:?}", list.shift());
    println!("list.shift() {:?}", list.shift());
    println!("list.shift() {:?}", list.shift());
    let vec: Vec<_> = list.into();
    println!("{:?}", vec);
}