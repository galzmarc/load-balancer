use load_balancer::Queue;

#[test]
fn test_queue() {
  let mut queue: Queue<u32> = Queue::new();
  queue.enqueue(1);
  queue.enqueue(2);
  queue.enqueue(3);

  assert_eq!(queue.dequeue(), Some(1));
  assert_eq!(queue.dequeue(), Some(2));
  assert_eq!(queue.dequeue(), Some(3));
  assert_eq!(queue.dequeue(), None);
}