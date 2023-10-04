/*  
    mod:
        Heap
    desc: 
        implementation of a heap data structure
    api:
        1. new()
        2. size()
        3. is_empty()
        4. peek()
        5. insert(item)
        6. remove()
*/
pub enum HeapType { Min, Max }

pub struct Heap<T>
where
    T: Ord + PartialOrd + Eq + PartialEq
{
    data: Vec<T>,
    h_type: HeapType
}

impl<T: Ord + Copy> Heap<T> {
    pub fn new(heap: HeapType) -> Self {
        // instantiates a new heap
        Self {
            data: Vec::new(),
            h_type: heap
        }
    }

    pub fn size(&self) -> usize {
        // returns the heap size
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        // returns true if the heap is empty, false otherwise
        self.data.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        // returns a reference to the root node
        self.data.get(0)
    }

    pub fn insert(&mut self, node: T) -> &mut Self {
        // inserts a node in the heap
        if self.is_empty() {
            self.data.push(node);
            return self
        }

        self.data.push(node);
        self.heapify_up(node, self.size()-1);
        
        self
    }

    pub fn remove(&mut self) -> T {
        // removes the root  node from the heap
        if self.is_empty() {
            panic!("Fatal: cannot remove from an empty heap");
        }

        let size = self.size();
        self.data.swap(0, size-1);

        let root = self.data.remove(size-1);
        self.heapify_down(0);

        root
    }

    fn heapify_up(&mut self, node: T, from: usize) {
        // maintains heap invariant from node to root
        let mut index = from;

        while index > 0 {
            let p_node = self.parent(index).expect("Fatal: failed to get parent node");
            let cmp = match self.h_type {
                HeapType::Min => |a: &T, b: &T| a < b,
                HeapType::Max => |a: &T, b: &T| a > b,
            };

            if cmp(&node, p_node) {
                self.data.swap(index, index / 2);  // swap the child with the parent
            } else {
                break;  // the child is in its final position
            }

            index /= 2;
        }
    }

    fn heapify_down(&mut self, from: usize) {
        // maintains heap invariant from root to base
        let size = self.size();
        let mut index = from;
        let mut i_node = index;  // the node we're interested in

        let cmp = match self.h_type {
            HeapType::Min => |a: &T, b: &T| a < b,
            HeapType::Max => |a: &T, b: &T| a > b,
        };

        while 2*index < size {
            let l_index = 2*index + 1;
            let r_index = 2*index + 2;

            if l_index < size {
                if let Some(l_child) = self.l_child(index) {
                    if cmp(l_child, &self.data[index]) {
                        i_node = l_index;
                        self.data.swap(index, i_node);
                    }
                }
            }

            if r_index < size {
                if let Some(r_child) = self.r_child(index) {
                    if cmp(r_child, &self.data[index]) {
                        i_node = r_index;
                        self.data.swap(index, i_node);
                    }
                }
            }

            if i_node == index {  // heap invariant is preserved
                break;
            }

            index = i_node;
        }
    }

    fn parent(&self, of_child: usize) -> Option<&T> {
        // returns the parent of the child at the index if present
        self.data.get(of_child / 2)
    }

    fn l_child(&self, of_parent: usize) -> Option<&T> {
        // returns the left child node of a parent if present
        self.data.get(2*of_parent + 1)
    }

    fn r_child(&self, of_parent: usize) -> Option<&T> {
        // returns the right child node of a parent if present
        self.data.get(2*of_parent + 2)
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::heap::{Heap, HeapType};

    #[test]
    fn test_min_heap() {
        let data = vec![1, -2, 2, 3, -1, 0];
        let mut min_heap: Heap<i32> = Heap::new(HeapType::Min);
        
        for item in data {
            min_heap.insert(item);
        }

        assert_eq!(min_heap.size(), 6);
        assert_eq!(*min_heap.peek().unwrap(), -2);
        assert_eq!(min_heap.remove(), -2);
        assert_eq!(*min_heap.peek().unwrap(), -1);
    }
}
