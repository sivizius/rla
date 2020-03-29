//! You have 2 Buckets A and B:
//! * one could hold 3 litres of Water,
//! * the other 5 litres of water.
//! You could do the following things:
//! * fill either of the buckets,
//! * emtpy either of the buckets or
//! * fill the content of one bucket to the other without overfilling.
//! The Goal is to have exactly 4 litres of Water in one of the buckets.
//! You could vary the capacity of each bucket.

#![allow(non_snake_case)]

use
{
  core::
  {
    fmt::Debug,
    ops::
    {
      Add,
      Sub,
    }
  },
};

/// A Bucket.
#[derive(Clone,Copy,Debug,PartialEq)]
struct        Bucket<T>
where
  T:                                    Add<Output=T> + Sub<Output=T> + Ord + PartialOrd + PartialEq + Copy + Clone + Debug,
{
  level:                                T,
  capacity:                             T,
}

/// Constructor for `Bucket`.
///
/// # Arguments
/// * `capacity`                        – capacity of this bucket.
fn            Bucket<T>
(
  capacity:                             T,
)
->  Bucket<T>
where
  T:                                    Add<Output=T> + Sub<Output=T> + Ord + PartialOrd + PartialEq + Copy + Clone + Debug,
{
  Bucket
  {
    level:                              capacity - capacity,
    capacity,
  }
}

impl<T>       Bucket<T>
where
  T:                                    Add<Output=T> + Sub<Output=T> + Ord + PartialOrd + PartialEq + Copy + Clone + Debug,
{
  /// Empty the Bucket.
  fn            empty
  (
    &self,
  )
  ->  Self
  {
    Self
    {
      level:                            self.capacity - self.capacity,
      capacity:                         self.capacity,
    }
  }

  /// Fill the Bucket to Maximum Level.
  fn            fill
  (
    &self,
  )
  ->  Self
  {
    Self
    {
      level:                            self.capacity,
      capacity:                         self.capacity,
    }
  }

  /// Decant the Bucket into Another.
  ///
  /// # Arguments
  /// * `this`                          – other bucket.
  fn            decant
  (
    &mut self,
    this:                               &mut Self,
  )
  {
    let     sum                         =   self.level + this.level;
    if sum > this.capacity
    {
      self.level                        =   self.level - ( this.capacity - this.level );
      this.level                        =   this.capacity;
    }
    else
    {
      self.level                        =   self.capacity - self.capacity;
      this.level                        =   sum;
    }
  }
}

/// Each Action that result in a Change of State.
#[derive(Clone,Copy,Debug)]
enum          Action
{
  /// Empty Bucket A.
  EmptyA,
  /// Empty Bucket B.
  EmptyB,
  /// Fill Bucket A.
  FillA,
  /// Fill Bucket B.
  FillB,
  /// Decant Bucket A into Bucket B.
  DecantA2B,
  /// Decant Bucket B into Bucket A.
  DecantB2A,
}

/// A possible State.
#[derive(Clone,Copy,Debug,PartialEq)]
struct        State<T>
where
  T:                                    Add<Output=T> + Sub<Output=T> + Ord + PartialOrd + PartialEq + Copy + Clone + Debug,
{
  goal:                                 T,
  bucketA:                              Bucket<T>,
  bucketB:                              Bucket<T>,
}

/// Constructor for a `State`.
///
/// # Arguments
/// * `` –
fn            State<T>
(
  goal:                                 T,
  bucketA:                              Bucket<T>,
  bucketB:                              Bucket<T>,
)
->  State<T>
where
  T:                                    Add<Output=T> + Sub<Output=T> + Ord + PartialOrd + PartialEq + Copy + Clone + Debug,
{
  State
  {
    goal,
    bucketA,
    bucketB,
  }
}

impl<T>       State<T>
where
  T:                                    Add<Output=T> + Sub<Output=T> + Ord + PartialOrd + PartialEq + Copy + Clone + Debug,
{
  /// Empty Bucket A.
  fn            emptyA
  (
    &self,
  )
  ->  Self
  {
    Self
    {
      goal:                             self.goal,
      bucketA:                          self.bucketA.empty(),
      bucketB:                          self.bucketB,
    }
  }

  /// Empty Bucket B.
  fn            emptyB
  (
    &self,
  )
  ->  Self
  {
    Self
    {
      goal:                             self.goal,
      bucketA:                          self.bucketA,
      bucketB:                          self.bucketB.empty(),
    }
  }

  /// Fill Bucket A.
  fn            fillA
  (
    &self,
  )
  ->  Self
  {
    Self
    {
      goal:                             self.goal,
      bucketA:                          self.bucketA.fill(),
      bucketB:                          self.bucketB,
    }
  }

  /// Fill Bucket B.
  fn            fillB
  (
    &self,
  )
  ->  Self
  {
    Self
    {
      goal:                             self.goal,
      bucketA:                          self.bucketA,
      bucketB:                          self.bucketB.fill(),
    }
  }

  /// Decant Bucket A into B.
  fn            decantA2B
  (
    &self,
  )
  ->  Self
  {
    let mut bucketA                     =   self.bucketA.clone();
    let mut bucketB                     =   self.bucketB.clone();
    bucketA.decant(&mut bucketB);
    Self
    {
      goal:                             self.goal,
      bucketA,
      bucketB,
    }
  }

  /// Decant Bucket B into A.
  fn            decantB2A
  (
    &self,
  )
  ->  Self
  {
    let mut bucketA                     =   self.bucketA.clone();
    let mut bucketB                     =   self.bucketB.clone();
    bucketB.decant(&mut bucketA);
    Self
    {
      goal:                             self.goal,
      bucketA,
      bucketB,
    }
  }
}

impl<T>       rla::Next<Action>         for State<T>
where
  T:                                    Add<Output=T> + Sub<Output=T> + Ord + PartialOrd + PartialEq + Copy + Clone + Debug,
{
  fn            done
  (
    &self,
  )
  ->  bool
  {
    ( self.bucketA.level == self.goal ) || ( self.bucketB.level == self.goal )
  }

  fn            fail
  (
    &self,
  )
  ->  Option  < String  >
  {
    if  self.bucketA.level <=  self.bucketA.capacity
    &&  self.bucketB.level <=  self.bucketB.capacity
    {
      None
    }
    else
    {
      Some
      (
        format!
        (
          "At least one Bucket is over its capacity:\n→A = {:?}\n→B = {:?}",
          self.bucketA,
          self.bucketB,
        )
      )
    }
  }

  fn            next
  (
    &self,
    input:                              Action,
  )
  ->  Self
  {
    match input
    {
      Action::EmptyA                    =>  self.emptyA(),
      Action::EmptyB                    =>  self.emptyB(),
      Action::FillA                     =>  self.fillA(),
      Action::FillB                     =>  self.fillB(),
      Action::DecantA2B                 =>  self.decantA2B(),
      Action::DecantB2A                 =>  self.decantB2A(),
    }
  }
}

fn main()
->  Result
    <
      (),
      String,
    >
{
  match rla::solve
        (
          State
          (
            4,
            Bucket  ( 3 ),
            Bucket  ( 5 ),
          ),
          vec!
          [
            Action::EmptyA,
            Action::EmptyB,
            Action::FillA,
            Action::FillB,
            Action::DecantA2B,
            Action::DecantB2A,
          ],
        )
  {
    Ok  ( path  )
    =>  {
          for
          (
            index,
            there,
          )                             in  path.iter().enumerate()
          {
            println!
            (
              "Step {}: {:?}",
              index,
              there,
            );
          }
          Ok  ( ( ) )
        },
    Err ( message )
    =>  {
          println!
          (
            "{:#}\n",
            message,
          );
          Err ( message )
        },
  }
}
