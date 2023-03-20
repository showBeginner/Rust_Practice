struct Bott<T> {
    element:Vec<T>,
}

impl<T> Bott<T>{

    fn new()->Bott<T>{
        Bott{ element: Vec::new()}
    }

    fn pop_element(&mut self){
        self.element.pop();
    }

    fn push_element(&mut self,temp:T){
        self.element.push(temp);
    }

    fn top_element(&self) -> Option<&T>{
        self.element.last()
    }
}


fn main() {
    let mut mystack = Bott::<i32>::new();

    mystack.push_element(1);

    match mystack.top_element() {
        Some(x) => println!("Top element: {}",x),
        None =>println!("Empty stack!"),
    }
    mystack.pop_element();
}
