use std::collections::VecDeque;
struct QueueSt<T> {
    element: VecDeque<T>,
}

impl<T> QueueSt<T>{

    fn new()->QueueSt<T>{
        QueueSt{ element: VecDeque::new()}
    }

    fn pop_element(&mut self){
        self.element.pop_front();
    }

    fn push_element(&mut self,temp:T){
        self.element.push_back(temp);
    }

    fn top_element(&self) -> Option<&T>{
        self.element.front()
    }
}


fn main() {
    let mut myqueue = QueueSt::<i32>::new();

    myqueue.push_element(1);

    match myqueue.top_element() {
        Some(x) => println!("Top element: {}",x),
        None =>println!("Empty queue!"),
    }
    myqueue.pop_element();
}
