# Backpressure

There's a second detail hidden in the previous example. The channel is processing messages *one at a time*, but as fast as it can. So you won't *ever* overwhelm the writer socket with concurrent writes. You haven't had to create any Mutexes or other locking mechanisms to protect the socket (and avoid borrow checker arguments).

This bridges into a topic called *backpressure*. Notice that whenever you create a channel, you specify a buffer size? Sending to the channel won't block until the buffer is full. Once the buffer is full, sending to the channel will block until there's room in the buffer. This is a great way to avoid overwhelming a slow consumer. (You can use `try_send` if you want to know that blocking will occur).

This provides a great way to handle bursts of activity, or provide a self-pacing mechanism. If the consumer is slow, the producer will slow down. If the consumer is fast, the producer can keep up.

Let's look at the `code/backpressure` example.

We're producing random numbers with a bunch of producers. Each is passed to a processing layer (it simply batches them together), before submitting to a consumer stage. The final stage has a simulated delay, to represent a slow consumer.

The key here is that each stage has a bounded channel, and backpressure is applied throughout the pipeline. The overall process system is self-regulating. This could be useful:

* If you are monitoring inputs and need to regulate the processing rate to what you can handle.
* If you are processing data from a network source, and need to avoid overwhelming your network connection.
* If you are processing data from a disk source, and need to avoid overwhelming your disk I/O.