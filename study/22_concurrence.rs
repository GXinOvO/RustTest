/*
    --TODO: Rust并发编程
        安全高效的处理并发是Rust诞生的目的之一，主要解决的是服务器高负载承受能力
        并发(concurrent)的概念是指程序不同的部分独立执行，这与并行(parallel)的概念容易
      混淆，并行强调的是"同时执行"。并发往往会造成并行。


    --TODO: 线程
        线程(thread)是一个程序中独立运行的一个部分。
        线程不同于进程(process)的地方是线程是程序以内的概念，程序往往是在一个进程中执行的。
        在有操作系统的环境中进程往往被交替地调度得以执行，线程则在进程以内由程序进行调度。
        由于线程并发很有可能出现并行的情况，所以在并行中可能遇到的死锁、延宕错误常出现于含
      有并发机制的程序
        为了解决这些问题，很多其他语言(如Java、C#)采用特殊的运行时(runtime)软件来协调资源，但这样
      无疑极大地降低了程序的执行效率。
        C/C++语言在操作系统的最底层也支持多线程，且语言本身以及其编译器不具备侦察和避免并行错误的
      能力，这对于开发者来说压力很大，开发者需要花费大量的精力避免发生错误。
        Rust不依靠运行时环境，这一点像C/C++一样。
        但Rust在语言本身就设计了包括所有权机制在内的手段来尽可能地把最常见的错误消灭在编译阶段，
      这一点其他语言不具备。但这不意味着我们编程的时候可以不小心，迄今为止由于并发造成的问题还没有在
      公共范围内得到完全解决，仍有可能出现错误，并发编程时尽量新校


        Rust中通过std::thread::spawn函数创建新线程
*/

/* 
use std::thread;
use std::time::Duration;

fn spawn_function()
{
  for i in 0..5 
  {
    println!("spawned thread print {}", i);
    thread::sleep(Duration::from_millis(1));
  }
}

fn main()
{
  // std::thread::spawn函数的参数是一个无参函数，但这个写法并不是推荐的写法，我们可以
  使用闭包来传递函数作为参数:
  thread::spawn(spawn_function);

  for i in 0..3
  {
    println!("main thread print {}", i);
    thread::sleep(Duration::from_millis(1));
  }
}
*/

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main()
{
  /*
    --TODO:
      闭包是可以保存进变量或作为参数传递给其他函数的匿名函数。闭包相当于Rust中的
    Lambda表达式，格式:
    | 参数1, 参数2, ... | -> 返回值类型 {
      // 函数体
    }
   */
  thread::spawn(|| {
    for i in 0..5 {
      println!("spawned thread print {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 0..3 {
    println!("main thread print {}", i);
    thread::sleep(Duration::from_millis(1));
  }

  let inc = |num: i32| -> i32 {
    num + 1
  };
  println!("inc(5) = {}", inc(5));

  /*
    --TODO:
      闭包可以省略类型声明使用Rust自动类型判断机制
   */
  let inc1 = |num| {
    num + 1
  };
  println!("inc1(5) = {}", inc1(5));

  /*
    --TODO: join方法
      join方法可以使子线程运行结束后再停止运行程序
   */
  let handle = thread::spawn(|| {
    for i in 0..5 {
      println!("handle spawned thread print {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 0..3 {
    println!("handle main thread print {}", i);
    thread::sleep(Duration::from_millis(1));
  }
  handle.join().unwrap();

  /*
    --TODO: move强制所有权转移
      下面代码在子线程中尝试使用当前函数的资源，这一定是错误的！因为所有权机制
    禁止这种危险情况的产生，他将破坏所有权机制销毁资源的一定性。我们可以使用闭
    包的move关键字来处理:
   */
  let s = "hello";
  
  /* 
    error:
  let hand = thread::spawn(|| {
    println!("{}", s);
  });
  */
  let hand = thread::spawn(move || {
    println!("{}", s);
  });
  hand.join().unwrap();


  /*
    --TODO: 消息传递
      Rust中一个实现消息传递并发的主要工具是通道(channel)，通道有两部分组成，一个发
    送者(transmitter)和一个接收者(reveiver)
      std::sync::mpsc包含了消息传递的方法
   */
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
  });
  let received = rx.recv().unwrap();
  println!("Got: {}", received);

}