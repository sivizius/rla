//! RLA is a set of helper-functions for formal specification
//!   to design, model, document, and verify programs,
//!   especially concurrent systems and distributed systems.

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

use
{
  core::
  {
    fmt::Debug,
  },
};

/// States that Implement this Trait can Step to Another State.
pub trait     Next<U>
{
  /// Am I done?
  fn            done
  (
    &self,
  )
  ->  bool;

  /// Did I fail?
  fn            fail
  (
    &self,
  )
  ->  Option  < String  >;

  /// Go to Next State.
  ///
  /// # Arguments
  /// * `input`                         – use this as hint.
  fn            next
  (
    &self,
    input:                              U,
  )
  ->  Self;
}

/// Node consisting of a `State` and arrows to other states.
#[derive(Debug)]
pub struct    Node<T>
where
  T:                                    Debug,
{
  reached:                              bool,
  state:                                T,
  arrows:                               Vec     < usize >,
}

/// A `Skip` is a `Step` but with indices.
#[derive(Clone,Copy,Debug)]
pub struct    Skip
{
  here:                                 usize,
  skip:                                 usize,
}

/// A `Step` states and its next actions.
#[derive(Clone,Copy,Debug)]
pub struct    Step<T,U>
{
  state:                               T,
  action:                              Option < U >,
}

/// Checks, if at least one element of the given `set` exists, where the given `condition` is true.
pub fn exists<T, R, F>
(
  set:                                  R,
  condition:                            F,
)
->  bool
where
  R:                                    Iterator  < Item  = T >,
  F:                                    Fn  ( T ) ->  bool,
{
  for value                             in  set
  {
    if  condition(value)
    {
      return true;
    }
  }
  false
}

/// Checks, if for all element of the given `set`, the given `condition` is true.
pub fn forAll<T, R, F>
(
  set:                                  R,
  condition:                            F,
)
->  bool
where
  R:                                    Iterator  < Item  = T >,
  F:                                    Fn  ( T ) ->  bool,
{
  for value                             in  set
  {
    if  !condition(value)
    {
      return false;
    }
  }
  true
}

/// Find a `Behaviour` that result in a successful `State`
///
/// # Arguments
/// `state`                             – initial state of the state machine,
/// `actions`                           – set of possible actions/inputs.
pub fn solve<T, U>
(
  state:                                T,
  actions:                              Vec < U >,
)
->  Result
    <
      Vec < Step  < T,  U > >,
      String,
    >
where
  T:                                    Next<U> + PartialEq + Copy + Debug,
  U:                                    Clone + Copy,
{
  let mut nodes
  =   vec!
      [
        Node
        {
          reached:                      false,
          state,
          arrows:                       Vec::new(),
        }
      ];
  let mut path                          =   Vec::new  ( ) as  Vec < Skip  >;
  let mut this
  =   Skip
      {
        here:                           0,
        skip:                           0,
      };
  'otter:
  loop
  {
    if      nodes [ this.here ].state.done  ( )
    {
      let mut path:                     Vec < Step  < T,  U > >
      =   path
            .iter()
            .map
            (
              | that |
              Step
              {
                state:                  nodes [ that.here ].state,
                action:                 Some  ( actions [ that.skip ] ),
              }
            )
            .collect();
      path.push
      (
        Step
        {
          state:                        nodes [ this.here ].state,
          action:                       None,
        }
      );
      break Ok  ( path  );
    }
    else if nodes [ this.here ].state.fail  ( ).is_some ( )
    {
      //  `solve` tries to find any path to a successful state,
      //    even if some chains of actions lead to failures,
      //  but it does not make sense to go from such states to a next state,
      //    even if this might be possible and result in an allowed state.
      //  `check` on the other side should fail here,
      //    but should ignore successful states.
      continue;
    }
    else
    {
      nodes [ this.here ].reached       =   true;
      for action                        in  &actions
      {
        //  Calculate next state from current state, if current action would be done.
        let     state                   =   nodes [ this.here ].state.next  ( *action );
        let mut there                   =   nodes.len();
        let mut unknown                 =   true;
        //  Do I already know that state?
        for
        (
          index,
          node,
        )                               in  nodes.iter().enumerate()
        {
          if  node.state  ==  state
          {
            //  Yes, state is not new to me.
            there                       =   index;
            unknown                     =   false;
            break;
          }
        }
        nodes [ this.here ].arrows.push ( there );
        //  Add unknown state to list of nodes.
        if  unknown
        {
          nodes.push
          (
            Node
            {
              reached:                  false,
              state,
              arrows:                   Vec::new(),
            }
          )
        }
      }
      //  Who is next?
      'inner:
      loop
      {
        //  is there any possible next state,
        //    that was not reached yet?
        for
        (
          skip,
          next,
        )
        in  nodes [ this.here ]
              .arrows
              .iter       (           )
              .skip       ( this.skip )
              .enumerate  (           )
        {
          if  !nodes  [ *next ].reached
          {
            this.skip                   =   skip;
            path.push ( this.clone  ( ) );
            this.here                   =   *next;
            this.skip                   =   0;
            break 'inner;
          }
        }
        //  dead end, go one step back and try again.
        if  let Some  ( previous  ) = path.pop()
        {
          this                          =   previous;
        }
        else
        {
          let mut message               =   "Found all reachable states, but none of them was a final state:\n".to_owned();
          for
          (
            index,
            node,
          )                             in  nodes.iter().enumerate()
          {
            message.push_str
            (
              &format!
              (
                "Node {}: {:?}, {:?}\n",
                index,
                node.state,
                node.arrows,
              )
            )
          }
          break 'otter Err  ( message )
        }
      }
    }
  }
}
