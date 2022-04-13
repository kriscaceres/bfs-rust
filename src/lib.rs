/* VecDeque -> "double-ended queue implemented with a growable ring buffer"
 * --meaning, it has O(1) enqueue/dequeue time
 *
 * structs are like MATLAB structs -> type agnostic, easily accessible fields,
 *                                      and simple.
 *
 * like in Java, T is a generic type that allows us to reuse the same code block
 *  for different variable types. this is important since Queues should support
 *  all data types.
 *
 *  impl should self-explain, but it allows us to define functions 
 *  and apply traits to structs.
 *
 *  .pop_front in dequeue() "returns an Option containing the first element
 *  of the VecDeque, or None if the VecDeque is empty
 *
 *  Option types represent optional values -> each Option is either:
 *      Some: contains a value
 *          OR
 *      None: does not contain a value
 *  we can use this for pattern matching like this:
 *  match result{
 *      Some(x) => foobar(),
 *      None => error()
 *  }
 *  In this case, .expect() uses the None result for custom error messages, or
 *  just returns if Some is returned.
 *
 *
 */



use std::collections::VecDeque;

struct Queue<T> {
    pub items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, v: T){
        self.items.push_back(v)
    }

    pub fn dequeue(&mut self) -> T {
        self.items
            .pop_front()
            .expect("Cannot dequeue from empty queue.")
    }

    pub fn is_empty(&self) -> bool{
        self.items.len() == 0
    }
}

type Vertex = Vec<u32>; //the node in the Graph
type Graph = Vec<Vertex>; //the list of all nodes adjacent to Vertex in Graph




