use std::{
    future::Future,
    time::{Duration, Instant},
};

pub struct TimedFuture<Fut: Future> {
    start: Option<Instant>,
    future: Fut,
}

impl<Fut: Future> Future for TimedFuture<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let start = self.start.get_or_insert(Instant::now());
        let mut inner_poll = std::pin::pin!(self.future).as_mut().poll(cx);
        let elapsed = self.start.unwrap().elapsed();

        match inner_poll {
            std::task::Poll::Ready(_) => todo!(),
            std::task::Poll::Pending => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
