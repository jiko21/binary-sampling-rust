use rand::Rng;

#[derive(Debug)]
pub enum BinaryTree {
  None,
  Node {
    val: usize,
    weight: f64,
    left: Box<BinaryTree>,
    right: Box<BinaryTree>,
  },
}

#[macro_export]
macro_rules! binary_tree {
  (val: $val:expr, weight: $weight:expr, left: $left:expr, right: $right:expr $(,)? ) => {
    BinaryTree::Node {
      val: $val,
      weight: $weight,
      left: Box::new($left),
      right: Box::new($right),
    }
  };
  (val: $val:expr, weight: $weight:expr, left: $left:expr $(,)? ) => {
    binary_tree! (
      val: $val,
      weight: $weight,
      left: Box::new($left),
      right: binary_tree!(),
    )
  };
  (val: $val:expr, weight: $weight:expr, right: $right:expr $(,)? ) => {
    binary_tree! (
      val: $val,
      weight: $weight,
      left: binary_tree!(),
      right: Box::new($right),
    )
  };

  (val: $val:expr, weight: $weight:expr $(,)? ) => {
    binary_tree!(
      val: $val,
      weight: $weight,
      left: binary_tree!(),
      right: binary_tree!(),
    )
  };
  () => {
    BinaryTree::None
  };
}

fn sampling(root: &BinaryTree) -> Result<usize, String> {
  let mut target = root;
  loop {
    match target {
      BinaryTree::Node{weight, val, ref left, ref right, .. } => {
        let a: f64 = rand::thread_rng().gen_range(0.0f64..*weight);
        match **left {
          BinaryTree::Node{weight, ..} => {
            if a < weight {
              target = left;
            } else {
              target = right;
            }
          }
          _ => {
            break;
          }
        }
      },
      _ => {
        println!("break");
        break;
      }
    }
  }
  match target {
    BinaryTree::Node{val, ..} => {
      Ok(*val)
    }
    _ => {
      Err("no value returned".to_string())
    }
  }
}

fn main() {
  let root = binary_tree! {
    val: 0,
    weight: 1.0,
    left: binary_tree! {
      val: 0,
      weight: 0.5,
      left: binary_tree! {
        val: 0,
        weight: 0.25,
      },
      right: binary_tree! {
        val: 1,
        weight: 0.25,
      },
    },
    right: binary_tree! {
      val: 1,
      weight: 0.5,
      left: binary_tree! {
        val: 2,
        weight: 0.25,
      },
      right: binary_tree! {
        val: 3,
        weight: 0.25,
      },
    },
  };
  let mut arr: [i32; 4] =[0, 0, 0, 0];
  for i in 0..10000000 {
    let mut target = &root;
    let i = sampling(&target);
    match i {
      Ok(i) => {
        arr[i] += 1;
      },
      Err(e) => {
        println!("{}", e);
      }
    };
    // loop {
      // let i = sampling(&target);
      // match i {
      //   Ok(i) => {
      //     arr[i] += 1;
      //   },
      //   Err(e) => {
      //     println!("{}", e);
      //   }
      // };
      //   match target {
      //     BinaryTree::Node{weight, val, left, right, .. } => {
      //       let a: f64 = rand::thread_rng().gen_range(0.0f64..*weight);
      //       match **left {
      //         BinaryTree::Node{weight, ..} => {
      //           if a < weight {
      //             target = left;
      //           } else {
      //             target = right;
      //           }
      //         }
      //         _ => {
      //           break;
      //         }
      //       }
      //     },
      //     _ => {
      //       break;
      //     }
      //   }
      // }
      // match target {
      //   BinaryTree::Node{val, ..} => {
      //     arr[*val] += 1;
      //   }
      //   _ => {
      //     println!("finish");
      //   }
      // }
    // }
  }
  println!("{:?}", arr);
}
