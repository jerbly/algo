// Part 1: Implement a circular buffer with enqueue, dequeue, peek_oldest, and peek_newest
// Part 2:
//   problem 1: given a stream of values, find the average rate.
//              print an error if the rate falls below some threshold at the moment of ANY
//              received value. Print at the moment we receive a new element where the
//              average has dropped. You DO NOT need to handle the case where the stream has
//              completely stopped.
//              the average is over at least 10 elements.
//  problem 2: put a sequence of values into the queue with timestamps. call them failures.
//             print a message at any point if the most recent 3 values are within 2 minutes.
#[derive(Debug)]
struct CircularBuffer<T> {
    buf: Box<[Option<T>]>,
    en_pos: usize,
    de_pos: usize,
}

impl<T> CircularBuffer<T> {
    fn new(size: usize) -> Self {
        assert!(size > 0);
        // size+1 to maintain a gap between en and de
        // Option is not Clone so you can't do vec![None; size+1]
        let mut v = Vec::new();
        v.resize_with(size + 1, || None); 
        Self {
            buf: v.into_boxed_slice(),
            en_pos: 0,
            de_pos: 0,
        }
    }
    fn enqueue(&mut self, item: T) {
        // if the next pos is de then enqueue should fail
        let next_pos = (self.en_pos + 1) % self.buf.len();
        if next_pos == self.de_pos {
            todo!();
        } else {
            self.buf[self.en_pos] = Some(item);
            self.en_pos = next_pos;
        }
    }
    fn dequeue(&mut self) -> Option<T> {
        if self.de_pos == self.en_pos {
            None
        } else {
            let item = self.buf[self.de_pos].take();
            self.de_pos = (self.de_pos + 1) % self.buf.len();
            item
        }
    }
    fn peek_oldest(&self) -> &Option<T> {
        &self.buf[self.de_pos]
    }
    fn peek_newest(&self) -> &Option<T> {
        let new_pos = if self.en_pos == 0 {
            self.buf.len() - 1
        } else {
            self.en_pos - 1
        };
        &self.buf[new_pos]
    }
}


#[test]
fn test_avg() {
    let mut c = CircularBuffer::new(10);
    // load with ten items
    for _ in 0..10 {
        c.enqueue(10);
    }
    let mut sum = 100;
    let mut avg = sum / 10;

    // now dequeue and enqueue as each item arrives
    let d = [2,10,10,15,15,2,0,2,2,2,0,0,0,0,0];
    for i in d {
        if let Some(item) = c.dequeue() {
            sum -= item;
        }
        c.enqueue(i);
        sum += i;
        avg = sum / 10;
        println!("sum:{sum}, avg:{avg}, below:{}",avg<5);
    }
}

#[test]
fn test_time() {
    let mut c = CircularBuffer::new(3);
    // load with items
    for i in 0..3 {
        c.enqueue(i*60);
    }
    let mut dif = 2;

    // now dequeue and enqueue as each item arrives
    let d = [4*60,5*60,6*60,7*60,8*60,9*60,10*60,10*60,11*60,12*60,13*60];
    for i in d {
        c.dequeue();
        c.enqueue(i);

        if let Some(item) = c.peek_oldest() {
            dif = i-item;            
        }
        println!("dif:{dif}, below:{}",dif<120);
    }
}

#[test]
fn test() {
    let mut c = CircularBuffer::new(3);
    c.enqueue(1);
    c.enqueue(2);
    c.enqueue(3);
    println!("{:?}", c);

    let d = c.dequeue();
    println!("{:?}", d);
    println!("{:?}", c);
    c.enqueue(4);
    let d = c.dequeue();
    println!("{:?}", d);
    println!("{:?}", c);

    let d = c.peek_newest();
    println!("New: {:?}", d);

    c.enqueue(5);
    let d = c.dequeue();
    println!("{:?}", d);
    println!("{:?}", c);
    c.enqueue(6);
    println!("{:?}", c);

    let d = c.peek_newest();
    println!("New: {:?}", d);

    let d = c.peek_oldest();
    println!("Old: {:?}", d);

    let d = c.dequeue();
    println!("{:?}", d);
    println!("{:?}", c);
    let d = c.dequeue();
    println!("{:?}", d);
    println!("{:?}", c);
    let d = c.dequeue();
    println!("{:?}", d);
    println!("{:?}", c);

    let d = c.dequeue();
    println!("{:?}", d);
    println!("{:?}", c);
}
