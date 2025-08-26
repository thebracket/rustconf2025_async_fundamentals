# The Rise of Threads

The first OS to really rely on threads (Simultaneous Multi-Threading - SMT) was IBM OS/360, in 1967.

What we recognize today as threads were formalized as POSIX threads in 1996. Linux began to support threads shortly after (as did Windows).

Threads are pre-emptively multitasked, in the same way as processes---they share the same memory space as the parent process, but the OS can interrupt them at any time to switch to another thread. The cost of context switching has gone down as CPUs have improved, but the OS still has to save and restore the state of each thread.

You can have thousands of threads, but the scheduling cost can become significant. Even if a thread is waiting for I/O, the OS has to manage its state.

## Selecting

Because of the cost of context switching, UNIX programs often used the `select` system (Berkeley sockets) to manage multiple I/O streams in a single thread:

```c
if (select(maxfd, &readfds, NULL, NULL, NULL) > 0) {
    if (FD_ISSET(sock1, &readfds)) {
        // sock1 is ready to read
        char buf[1024];
        int n = read(sock1, buf, sizeof(buf));
        printf("sock1: read %d bytes\n", n);
    }
    if (FD_ISSET(sock2, &readfds)) {
        // sock2 is ready to read
        char buf[1024];
        int n = read(sock2, buf, sizeof(buf));
        printf("sock2: read %d bytes\n", n);
    }
}
```

The *thread* blocks on the `select` call, asking the scheduler to wake it up when one of the sockets is ready to read. So this *is* async in a primitive form. Once again, the nerd-fight continued!