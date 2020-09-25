use core::fmt;
use core::future::Future;
use core::iter::FromIterator;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::boxed::Box;
use std::vec::Vec;
use std::io::{stdout, Stdout, Write};
use futures::future::{MaybeDone};

// This code is based on the source of https://docs.rs/futures/0.3.5/futures/future/struct.JoinAll.html

pub struct JoinAllProgress<F>
where
    F: Future,
{
    elems: Pin<Box<[MaybeDone<F>]>>,
    stdout: Stdout,
}

// IMPROVE: [`FuturesUnordered`][crate::stream::FuturesUnordered] APIs is more
// powerful - it can poll only futures that have been woken. Investigate.
pub fn join_all_progress<I>(i: I) -> JoinAllProgress<I::Item>
where
    I: IntoIterator,
    I::Item: Future,
{
    let elems: Box<[_]> = i.into_iter().map(MaybeDone::Future).collect();
    JoinAllProgress {
        elems: elems.into(),
        stdout: stdout()
    }
}

impl<F> Future for JoinAllProgress<F>
where
    F: Future,
{
    type Output = Vec<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut all_done = true;

        let mut tmp = vec![false; self.elems.len()];
        for (i ,elem) in iter_pin_mut(self.elems.as_mut()).enumerate() {
            if elem.poll(cx).is_pending() {
                all_done = false;
            } else {
                tmp[i] = true;
            }
        }

        let num_ready = tmp.iter().fold(0, |acc,e| if *e { acc + 1 } else { acc });

        print!("\rReady {}/{}", num_ready, tmp.len());
        self.stdout.flush().unwrap();

        if all_done {
            println!();
            let mut elems = mem::replace(&mut self.elems, Box::pin([]));
            let result = iter_pin_mut(elems.as_mut())
                .map(|e| e.take_output().unwrap())
                .collect();
            Poll::Ready(result)
        } else {
            Poll::Pending
        }
    }
}

fn iter_pin_mut<T>(slice: Pin<&mut [T]>) -> impl Iterator<Item = Pin<&mut T>> {
    // Safety: `std` _could_ make this unsound if it were to decide Pin's
    // invariants aren't required to transmit through slices. Otherwise this has
    // the same safety as a normal field pin projection.
    unsafe { slice.get_unchecked_mut() }
        .iter_mut()
        .map(|t| unsafe { Pin::new_unchecked(t) })
}

impl<F> fmt::Debug for JoinAllProgress<F>
where
    F: Future + fmt::Debug,
    F::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinAll")
            .field("elems", &self.elems)
            .finish()
    }
}

impl<F: Future> FromIterator<F> for JoinAllProgress<F> {
    fn from_iter<T: IntoIterator<Item = F>>(iter: T) -> Self {
        join_all_progress(iter)
    }
}
